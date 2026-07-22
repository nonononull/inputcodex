#!/usr/bin/env python3
"""inputcodex 上游变化监控。"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import re
import sys
import urllib.error
import urllib.parse
import urllib.request
from dataclasses import asdict, dataclass
from datetime import datetime, timezone
from pathlib import Path
from typing import Any, Iterable


EXPECTED_UPSTREAM_REPOSITORY = "BigPizzaV3/CodexPlusPlus"
EXPECTED_TARGET_REPOSITORY = "nonononull/inputcodex"
DEFAULT_SOURCE_LOCK = Path("upstream/source-lock.json")
ISSUE_LABELS = ["type:upstream-watch", "gate:2"]
STATE_MARKER = "<!-- inputcodex:upstream-watch:state:v1 -->"
STATE_DATA_START = "<!-- inputcodex:upstream-watch:state-data:v1"
STATE_DATA_END = "-->"
ALERT_MARKERS = {
    "new-release": "<!-- inputcodex:upstream-watch:alert:new-release:v1 -->",
    "release-tag-drift": "<!-- inputcodex:upstream-watch:alert:release-tag-drift:v1 -->",
    "release-metadata-change": "<!-- inputcodex:upstream-watch:alert:release-metadata-change:v1 -->",
    "main-change": "<!-- inputcodex:upstream-watch:alert:main-change:v1 -->",
    "monitor-error": "<!-- inputcodex:upstream-watch:alert:monitor-error:v1 -->",
}
EVENT_TITLES = {
    "new-release": "[Upstream Watch] 发现新的正式 Release",
    "release-tag-drift": "[Upstream Watch] Release 标签提交发生变化",
    "release-metadata-change": "[Upstream Watch] Release 元数据发生变化",
    "main-change": "[Upstream Watch] 上游 main 变化预警",
    "monitor-error": "[Upstream Watch] 监控执行异常",
}
EVENT_CHANGE_TYPES = {
    "new-release": "新正式 Release",
    "release-tag-drift": "Release 标签提交变化",
    "release-metadata-change": "Release 元数据变化",
    "main-change": "main 变化预警",
    "monitor-error": "监控异常",
}
SHA_PATTERN = re.compile(r"^[0-9a-f]{40}$")
FINGERPRINT_PATTERN = re.compile(r"^[0-9a-f]{64}$")
TAG_PATTERN = re.compile(r"^[A-Za-z0-9][A-Za-z0-9._+-]{0,127}$")
REPOSITORY_PATTERN = re.compile(r"^[A-Za-z0-9_.-]+/[A-Za-z0-9_.-]+$")
FINGERPRINT_MARKER_PATTERN = re.compile(
    r"^<!-- inputcodex:upstream-watch:fingerprint:([0-9a-f]{64}) -->$",
    re.MULTILINE,
)


class MonitorError(RuntimeError):
    """监控输入、状态或 GitHub API 不满足失败合同。"""


@dataclass(frozen=True)
class Baseline:
    upstream_repository: str
    release_tag: str
    release_published_at: str
    release_url: str
    release_commit: str

    def __post_init__(self) -> None:
        validate_repository(self.upstream_repository, EXPECTED_UPSTREAM_REPOSITORY)
        validate_tag(self.release_tag)
        validate_timestamp(self.release_published_at)
        validate_release_url(self.release_url, self.upstream_repository, self.release_tag)
        validate_sha(self.release_commit)


@dataclass(frozen=True)
class Observation:
    observed_at: str
    latest_release_tag: str
    latest_release_published_at: str
    latest_release_url: str
    latest_release_commit: str
    locked_release_tag: str
    locked_release_commit: str
    main_commit: str

    def __post_init__(self) -> None:
        validate_timestamp(self.observed_at)
        validate_tag(self.latest_release_tag)
        validate_timestamp(self.latest_release_published_at)
        validate_release_url(
            self.latest_release_url,
            EXPECTED_UPSTREAM_REPOSITORY,
            self.latest_release_tag,
        )
        validate_sha(self.latest_release_commit)
        validate_tag(self.locked_release_tag)
        validate_sha(self.locked_release_commit)
        validate_sha(self.main_commit)


@dataclass(frozen=True)
class Event:
    kind: str
    fingerprint: str
    payload: dict[str, Any]
    observed_at: str


@dataclass(frozen=True)
class UpsertResult:
    action: str
    issue_number: int
    issue_url: str


@dataclass(frozen=True)
class ExecutionSummary:
    event_count: int
    event_actions: tuple[str, ...]
    state_action: str


def validate_repository(value: str, expected: str | None = None) -> str:
    if not isinstance(value, str) or not REPOSITORY_PATTERN.fullmatch(value):
        raise MonitorError("仓库名格式无效。")
    if expected is not None and value != expected:
        raise MonitorError(f"仓库名必须为 {expected}。")
    return value


def validate_tag(value: str) -> str:
    if not isinstance(value, str) or not TAG_PATTERN.fullmatch(value):
        raise MonitorError("Release tag 格式无效。")
    return value


def validate_sha(value: str) -> str:
    if not isinstance(value, str) or not SHA_PATTERN.fullmatch(value):
        raise MonitorError("Git 提交 SHA 必须为 40 位小写十六进制。")
    return value


def validate_timestamp(value: str) -> str:
    if not isinstance(value, str) or not value.endswith("Z"):
        raise MonitorError("时间必须使用 UTC RFC3339 Z 格式。")
    try:
        parsed = datetime.fromisoformat(value.removesuffix("Z") + "+00:00")
    except ValueError as error:
        raise MonitorError("时间不是有效的 UTC RFC3339 值。") from error
    if parsed.utcoffset() != timezone.utc.utcoffset(parsed):
        raise MonitorError("时间必须使用 UTC。")
    return value


def validate_release_url(value: str, repository: str, tag: str) -> str:
    if not isinstance(value, str):
        raise MonitorError("Release URL 缺失。")
    parsed = urllib.parse.urlparse(value)
    expected_path = f"/{repository}/releases/tag/{tag}"
    if parsed.scheme != "https" or parsed.netloc != "github.com" or parsed.path != expected_path:
        raise MonitorError("Release URL 与仓库或 tag 不一致。")
    if parsed.params or parsed.query or parsed.fragment:
        raise MonitorError("Release URL 不得包含参数、查询或片段。")
    return value


def baseline_from_data(data: Any) -> Baseline:
    if not isinstance(data, dict) or data.get("schema_version") != "inputcodex.source-lock.v1":
        raise MonitorError("source-lock schema_version 无效。")
    snapshot = data.get("snapshot")
    if not isinstance(snapshot, dict):
        raise MonitorError("source-lock 缺少 snapshot 对象。")
    required = {
        "repository",
        "release_tag",
        "release_published_at",
        "release_url",
        "commit",
    }
    if not required.issubset(snapshot):
        raise MonitorError("source-lock snapshot 字段不完整。")
    return Baseline(
        upstream_repository=snapshot["repository"],
        release_tag=snapshot["release_tag"],
        release_published_at=snapshot["release_published_at"],
        release_url=snapshot["release_url"],
        release_commit=snapshot["commit"],
    )


def load_baseline(path: Path) -> Baseline:
    try:
        data = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, UnicodeError, json.JSONDecodeError) as error:
        raise MonitorError(f"无法读取有效的 source-lock：{path.as_posix()}") from error
    return baseline_from_data(data)


def utc_now() -> str:
    return datetime.now(timezone.utc).replace(microsecond=0).isoformat().replace("+00:00", "Z")


def stable_fingerprint(kind: str, payload: dict[str, Any]) -> str:
    encoded = json.dumps(
        {"kind": kind, "payload": payload},
        ensure_ascii=False,
        sort_keys=True,
        separators=(",", ":"),
    ).encode("utf-8")
    return hashlib.sha256(encoded).hexdigest()


def make_event(kind: str, payload: dict[str, Any], observed_at: str) -> Event:
    if kind not in ALERT_MARKERS or kind == "monitor-error":
        raise MonitorError("未知上游变化事件。")
    return Event(
        kind=kind,
        fingerprint=stable_fingerprint(kind, payload),
        payload=payload,
        observed_at=observed_at,
    )


def decide_events(
    baseline: Baseline,
    previous: Observation | None,
    current: Observation,
) -> list[Event]:
    reference_tag = previous.latest_release_tag if previous else baseline.release_tag
    reference_commit = previous.latest_release_commit if previous else baseline.release_commit
    reference_published_at = (
        previous.latest_release_published_at if previous else baseline.release_published_at
    )
    reference_url = previous.latest_release_url if previous else baseline.release_url
    events: list[Event] = []

    if current.latest_release_tag != reference_tag:
        events.append(
            make_event(
                "new-release",
                {
                    "previous_tag": reference_tag,
                    "previous_commit": reference_commit,
                    "current_tag": current.latest_release_tag,
                    "current_commit": current.latest_release_commit,
                    "current_published_at": current.latest_release_published_at,
                    "current_url": current.latest_release_url,
                },
                current.observed_at,
            )
        )

    tag_drifts: list[dict[str, str]] = []
    if current.locked_release_commit != baseline.release_commit:
        tag_drifts.append(
            {
                "scope": "frozen-baseline",
                "tag": baseline.release_tag,
                "expected_commit": baseline.release_commit,
                "actual_commit": current.locked_release_commit,
            }
        )
    if (
        current.latest_release_tag == reference_tag
        and current.latest_release_commit != reference_commit
        and not (
            current.latest_release_tag == baseline.release_tag
            and current.latest_release_commit == current.locked_release_commit
        )
    ):
        tag_drifts.append(
            {
                "scope": "latest-release",
                "tag": current.latest_release_tag,
                "expected_commit": reference_commit,
                "actual_commit": current.latest_release_commit,
            }
        )
    if tag_drifts:
        events.append(
            make_event(
                "release-tag-drift",
                {"drifts": tag_drifts},
                current.observed_at,
            )
        )

    if current.latest_release_tag == reference_tag:
        metadata_changes: dict[str, dict[str, str]] = {}
        if current.latest_release_published_at != reference_published_at:
            metadata_changes["published_at"] = {
                "before": reference_published_at,
                "after": current.latest_release_published_at,
            }
        if current.latest_release_url != reference_url:
            metadata_changes["url"] = {"before": reference_url, "after": current.latest_release_url}
        if metadata_changes:
            events.append(
                make_event(
                    "release-metadata-change",
                    {"tag": current.latest_release_tag, "changes": metadata_changes},
                    current.observed_at,
                )
            )

    if previous is not None and current.main_commit != previous.main_commit:
        events.append(
            make_event(
                "main-change",
                {
                    "before": previous.main_commit,
                    "after": current.main_commit,
                    "compare_url": (
                        f"https://github.com/{baseline.upstream_repository}/compare/"
                        f"{previous.main_commit}...{current.main_commit}"
                    ),
                },
                current.observed_at,
            )
        )

    return events


def observation_payload(observation: Observation) -> dict[str, str]:
    return asdict(observation)


def same_material_observation(first: Observation, second: Observation) -> bool:
    first_data = observation_payload(first)
    second_data = observation_payload(second)
    first_data.pop("observed_at")
    second_data.pop("observed_at")
    return first_data == second_data


def render_state_body(observation: Observation) -> str:
    state_json = json.dumps(observation_payload(observation), ensure_ascii=False, sort_keys=True)
    return (
        f"{STATE_MARKER}\n"
        "# inputcodex 上游监控状态\n\n"
        "本 Issue 由 `.github/workflows/upstream-watch.yml` 自动维护，只保存最近一次产生物质变化的成功观察值。"
        "请勿手工编辑、复制机器标记或将其作为功能同步授权。\n\n"
        f"- 最近记录时间：`{observation.observed_at}`\n"
        f"- 最新正式 Release：`{observation.latest_release_tag}`\n"
        f"- 最新 Release 提交：`{observation.latest_release_commit}`\n"
        f"- 冻结 Release tag：`{observation.locked_release_tag}`\n"
        f"- 上游 main：`{observation.main_commit}`\n\n"
        f"{STATE_DATA_START}\n{state_json}\n{STATE_DATA_END}\n"
    )


def parse_state_body(body: str) -> Observation:
    if first_line(body) != STATE_MARKER:
        raise MonitorError("状态 Issue 缺少精确机器标记。")
    pattern = re.compile(
        re.escape(STATE_DATA_START) + r"\n(\{[^\n]+\})\n" + re.escape(STATE_DATA_END)
    )
    matches = pattern.findall(body)
    if len(matches) != 1:
        raise MonitorError("状态 Issue 的机器数据缺失或重复。")
    try:
        data = json.loads(matches[0])
    except json.JSONDecodeError as error:
        raise MonitorError("状态 Issue 的机器数据不是有效 JSON。") from error
    expected_fields = set(Observation.__dataclass_fields__)
    if not isinstance(data, dict) or set(data) != expected_fields:
        raise MonitorError("状态 Issue 的机器数据字段不完整或包含未知字段。")
    return Observation(**data)


def first_line(body: Any) -> str:
    if not isinstance(body, str) or not body:
        return ""
    return body.splitlines()[0]


def find_unique_issue(issues: Iterable[dict[str, Any]], marker: str) -> dict[str, Any] | None:
    matches = [issue for issue in issues if first_line(issue.get("body")) == marker]
    if len(matches) > 1:
        numbers = ", ".join(str(issue.get("number", "?")) for issue in matches)
        raise MonitorError(f"机器标记重复，拒绝任意选择 Issue：{numbers}")
    return matches[0] if matches else None


def fingerprint_marker(fingerprint: str) -> str:
    if not FINGERPRINT_PATTERN.fullmatch(fingerprint):
        raise MonitorError("事件指纹必须为 64 位小写十六进制。")
    return f"<!-- inputcodex:upstream-watch:fingerprint:{fingerprint} -->"


def issue_fingerprint(issue: dict[str, Any]) -> str | None:
    body = issue.get("body")
    if not isinstance(body, str):
        return None
    matches = FINGERPRINT_MARKER_PATTERN.findall(body)
    if len(matches) > 1:
        raise MonitorError("告警 Issue 包含重复事件指纹。")
    return matches[0] if matches else None


def require_issue_number(issue: dict[str, Any]) -> int:
    number = issue.get("number")
    if not isinstance(number, int) or number <= 0:
        raise MonitorError("GitHub Issue 缺少有效编号。")
    return number


def issue_url(issue: dict[str, Any], repository: str) -> str:
    number = require_issue_number(issue)
    value = issue.get("html_url")
    expected = f"https://github.com/{repository}/issues/{number}"
    if not isinstance(value, str) or value != expected:
        raise MonitorError("GitHub Issue URL 与目标仓库或编号不一致。")
    return value


def upsert_machine_issue(
    client: Any,
    repository: str,
    issues: list[dict[str, Any]],
    marker: str,
    fingerprint: str,
    title: str,
    body: str,
) -> UpsertResult:
    validate_repository(repository, EXPECTED_TARGET_REPOSITORY)
    if marker not in ALERT_MARKERS.values():
        raise MonitorError("拒绝写入未知机器标记。")
    fingerprint_line = fingerprint_marker(fingerprint)
    existing = find_unique_issue(issues, marker)
    if existing is not None and issue_fingerprint(existing) == fingerprint:
        return UpsertResult("unchanged", require_issue_number(existing), issue_url(existing, repository))
    rendered = f"{marker}\n{fingerprint_line}\n\n{body.strip()}\n"
    if existing is None:
        created = client.create_issue(repository, title, rendered, ISSUE_LABELS)
        return UpsertResult("created", require_issue_number(created), issue_url(created, repository))
    updated = client.update_issue(
        repository,
        require_issue_number(existing),
        title,
        rendered,
        "open",
    )
    return UpsertResult("updated", require_issue_number(updated), issue_url(updated, repository))


def render_event_body(event: Event) -> str:
    evidence = json.dumps(event.payload, ensure_ascii=False, sort_keys=True, indent=2)
    return (
        f"## {EVENT_CHANGE_TYPES[event.kind]}\n\n"
        f"- 观察时间：`{event.observed_at}`\n"
        f"- 事件指纹：`{event.fingerprint}`\n\n"
        "### 可复核证据\n\n"
        f"```json\n{evidence}\n```\n\n"
        "### 对 inputcodex 的影响\n\n"
        "本 Issue 只负责预警和分流，不自动修改 `upstream/`、产品源码、Release 或 Ruleset。"
        "项目所有者应根据变化类型决定是否建立独立 upstream-sync、feature-parity 或 parity-exception Issue。\n\n"
        "### 边界确认\n\n"
        "- [x] 最新正式 Release 是功能真源；上游 `main` 仅作为变化预警源。\n"
        "- [x] 本 Issue 不构成自动同步、实现或合并授权。\n"
    )


def upsert_state_issue(
    client: Any,
    repository: str,
    state_issue: dict[str, Any] | None,
    previous: Observation | None,
    current: Observation,
) -> UpsertResult:
    if (
        state_issue is not None
        and previous is not None
        and same_material_observation(previous, current)
        and state_issue.get("state") == "open"
    ):
        return UpsertResult(
            "unchanged",
            require_issue_number(state_issue),
            issue_url(state_issue, repository),
        )
    title = "[Upstream Watch] 自动监控状态（请勿手工编辑）"
    body = render_state_body(current)
    if state_issue is None:
        created = client.create_issue(repository, title, body, ISSUE_LABELS)
        return UpsertResult("created", require_issue_number(created), issue_url(created, repository))
    updated = client.update_issue(
        repository,
        require_issue_number(state_issue),
        title,
        body,
        "open",
    )
    return UpsertResult("updated", require_issue_number(updated), issue_url(updated, repository))


def execute_monitor(
    client: Any,
    target_repository: str,
    baseline: Baseline,
    current: Observation,
) -> ExecutionSummary:
    validate_repository(target_repository, EXPECTED_TARGET_REPOSITORY)
    issues = client.list_issues(target_repository)
    state_issue = find_unique_issue(issues, STATE_MARKER)
    previous = parse_state_body(state_issue["body"]) if state_issue is not None else None
    events = decide_events(baseline, previous, current)
    actions: list[str] = []
    for event in events:
        result = upsert_machine_issue(
            client=client,
            repository=target_repository,
            issues=issues,
            marker=ALERT_MARKERS[event.kind],
            fingerprint=event.fingerprint,
            title=EVENT_TITLES[event.kind],
            body=render_event_body(event),
        )
        actions.append(f"{event.kind}:{result.action}:#{result.issue_number}")
    state_result = upsert_state_issue(
        client=client,
        repository=target_repository,
        state_issue=state_issue,
        previous=previous,
        current=current,
    )
    return ExecutionSummary(len(events), tuple(actions), state_result.action)


class GitHubClient:
    def __init__(self, token: str, api_url: str = "https://api.github.com") -> None:
        if not token or token.isspace():
            raise MonitorError("GITHUB_TOKEN 缺失。")
        parsed = urllib.parse.urlparse(api_url)
        if parsed.scheme != "https" or parsed.netloc != "api.github.com" or parsed.path not in {"", "/"}:
            raise MonitorError("GitHub API 地址必须为 https://api.github.com。")
        self._token = token
        self._api_url = "https://api.github.com"

    def _request_json(self, method: str, path: str, payload: Any = None) -> Any:
        if not path.startswith("/"):
            raise MonitorError("GitHub API 路径必须以 / 开头。")
        data = None
        headers = {
            "Accept": "application/vnd.github+json",
            "Authorization": f"Bearer {self._token}",
            "User-Agent": "inputcodex-upstream-watch/1",
            "X-GitHub-Api-Version": "2022-11-28",
        }
        if payload is not None:
            data = json.dumps(payload, ensure_ascii=False).encode("utf-8")
            headers["Content-Type"] = "application/json"
        request = urllib.request.Request(
            self._api_url + path,
            data=data,
            headers=headers,
            method=method,
        )
        try:
            with urllib.request.urlopen(request, timeout=30) as response:
                raw = response.read()
        except urllib.error.HTTPError as error:
            try:
                detail = json.loads(error.read(4096).decode("utf-8", errors="replace")).get("message")
            except (AttributeError, json.JSONDecodeError):
                detail = None
            suffix = f"：{detail}" if isinstance(detail, str) else ""
            raise MonitorError(f"GitHub API HTTP {error.code}{suffix}") from error
        except urllib.error.URLError as error:
            raise MonitorError(f"GitHub API 网络错误：{error.reason}") from error
        try:
            return json.loads(raw.decode("utf-8")) if raw else None
        except (UnicodeError, json.JSONDecodeError) as error:
            raise MonitorError("GitHub API 返回了无效 JSON。") from error

    def get(self, path: str) -> Any:
        return self._request_json("GET", path)

    def list_issues(self, repository: str) -> list[dict[str, Any]]:
        validate_repository(repository, EXPECTED_TARGET_REPOSITORY)
        issues: list[dict[str, Any]] = []
        for page in range(1, 101):
            data = self.get(f"/repos/{repository}/issues?state=all&per_page=100&page={page}")
            if not isinstance(data, list):
                raise MonitorError("GitHub Issues API 响应不是数组。")
            if any(not isinstance(item, dict) for item in data):
                raise MonitorError("GitHub Issues API 包含无效 Issue 条目。")
            page_issues = [item for item in data if "pull_request" not in item]
            issues.extend(page_issues)
            if len(data) < 100:
                return issues
        raise MonitorError("GitHub Issue 分页超过安全上限。")

    def create_issue(
        self,
        repository: str,
        title: str,
        body: str,
        labels: list[str],
    ) -> dict[str, Any]:
        data = self._request_json(
            "POST",
            f"/repos/{repository}/issues",
            {"title": title, "body": body, "labels": labels},
        )
        if not isinstance(data, dict):
            raise MonitorError("创建 Issue 的 API 响应无效。")
        return data

    def update_issue(
        self,
        repository: str,
        number: int,
        title: str,
        body: str,
        state: str,
    ) -> dict[str, Any]:
        if state not in {"open", "closed"}:
            raise MonitorError("Issue state 只能为 open 或 closed。")
        data = self._request_json(
            "PATCH",
            f"/repos/{repository}/issues/{number}",
            {"title": title, "body": body, "state": state},
        )
        if not isinstance(data, dict):
            raise MonitorError("更新 Issue 的 API 响应无效。")
        return data


def require_dict(value: Any, context: str) -> dict[str, Any]:
    if not isinstance(value, dict):
        raise MonitorError(f"{context} 响应不是对象。")
    return value


def require_string(data: dict[str, Any], field: str, context: str) -> str:
    value = data.get(field)
    if not isinstance(value, str) or not value:
        raise MonitorError(f"{context} 缺少字符串字段 {field}。")
    return value


def resolve_ref_commit(client: GitHubClient, repository: str, ref_kind: str, ref: str) -> str:
    encoded_ref = urllib.parse.quote(ref, safe="")
    data = require_dict(client.get(f"/repos/{repository}/git/ref/{ref_kind}/{encoded_ref}"), "Git ref")
    target = require_dict(data.get("object"), "Git ref object")
    for _ in range(5):
        object_type = require_string(target, "type", "Git ref object")
        sha = validate_sha(require_string(target, "sha", "Git ref object"))
        if object_type == "commit":
            return sha
        if object_type != "tag":
            raise MonitorError(f"Git ref 指向不支持的对象类型：{object_type}")
        tag_data = require_dict(client.get(f"/repos/{repository}/git/tags/{sha}"), "Git tag")
        target = require_dict(tag_data.get("object"), "Git tag object")
    raise MonitorError("Git tag 解引用超过安全上限。")


def fetch_observation(client: GitHubClient, baseline: Baseline) -> Observation:
    release = require_dict(
        client.get(f"/repos/{baseline.upstream_repository}/releases/latest"),
        "latest release",
    )
    tag = validate_tag(require_string(release, "tag_name", "latest release"))
    published_at = validate_timestamp(require_string(release, "published_at", "latest release"))
    url = validate_release_url(
        require_string(release, "html_url", "latest release"),
        baseline.upstream_repository,
        tag,
    )
    return Observation(
        observed_at=utc_now(),
        latest_release_tag=tag,
        latest_release_published_at=published_at,
        latest_release_url=url,
        latest_release_commit=resolve_ref_commit(client, baseline.upstream_repository, "tags", tag),
        locked_release_tag=baseline.release_tag,
        locked_release_commit=resolve_ref_commit(
            client,
            baseline.upstream_repository,
            "tags",
            baseline.release_tag,
        ),
        main_commit=resolve_ref_commit(client, baseline.upstream_repository, "heads", "main"),
    )


def safe_error_text(error: BaseException, token: str | None = None) -> str:
    text = f"{type(error).__name__}: {error}".replace("\r", " ").replace("\n", " ")
    if token:
        text = text.replace(token, "***")
    return text[:1000]


def report_monitor_error(
    client: GitHubClient,
    repository: str,
    error_text: str,
    observed_at: str,
) -> None:
    payload = {"error": error_text, "observed_at": observed_at}
    fingerprint = stable_fingerprint("monitor-error", {"error": error_text})
    body = (
        f"## {EVENT_CHANGE_TYPES['monitor-error']}\n\n"
        f"- 观察时间：`{observed_at}`\n"
        f"- 错误摘要：`{error_text}`\n"
        f"- 事件指纹：`{fingerprint}`\n\n"
        "本次扫描明确失败，不能解释为“没有上游变化”。请先确定 API、权限、响应结构或状态 Issue 的根因，"
        "再通过关联 Issue/PR 修复并 Fresh 验证。\n\n"
        f"```json\n{json.dumps(payload, ensure_ascii=False, sort_keys=True, indent=2)}\n```\n"
    )
    issues = client.list_issues(repository)
    upsert_machine_issue(
        client=client,
        repository=repository,
        issues=issues,
        marker=ALERT_MARKERS["monitor-error"],
        fingerprint=fingerprint,
        title=EVENT_TITLES["monitor-error"],
        body=body,
    )


def parse_args(argv: list[str] | None = None) -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="inputcodex 上游变化监控")
    parser.add_argument("--source-lock", type=Path, default=DEFAULT_SOURCE_LOCK)
    parser.add_argument("--validate-only", action="store_true")
    return parser.parse_args(argv)


def main(argv: list[str] | None = None) -> int:
    args = parse_args(argv)
    try:
        baseline = load_baseline(args.source_lock)
        if args.validate_only:
            print(
                json.dumps(
                    {
                        "status": "valid",
                        "upstream_repository": baseline.upstream_repository,
                        "release_tag": baseline.release_tag,
                        "release_commit": baseline.release_commit,
                    },
                    ensure_ascii=False,
                    sort_keys=True,
                )
            )
            return 0
        target_repository = validate_repository(
            os.environ.get("GITHUB_REPOSITORY", ""),
            EXPECTED_TARGET_REPOSITORY,
        )
        token = os.environ.get("GITHUB_TOKEN", "")
        client = GitHubClient(token)
        current = fetch_observation(client, baseline)
        summary = execute_monitor(client, target_repository, baseline, current)
        print(json.dumps(asdict(summary), ensure_ascii=False, sort_keys=True))
        return 0
    except Exception as error:
        token = os.environ.get("GITHUB_TOKEN", "")
        error_text = safe_error_text(error, token)
        print(f"UPSTREAM_WATCH_FAILED: {error_text}", file=sys.stderr)
        if not args.validate_only and token and os.environ.get("GITHUB_REPOSITORY") == EXPECTED_TARGET_REPOSITORY:
            try:
                report_monitor_error(
                    GitHubClient(token),
                    EXPECTED_TARGET_REPOSITORY,
                    error_text,
                    utc_now(),
                )
            except Exception as report_error:
                print(
                    f"UPSTREAM_WATCH_ERROR_REPORT_FAILED: {safe_error_text(report_error, token)}",
                    file=sys.stderr,
                )
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
