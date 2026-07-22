import importlib.util
import re
import sys
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[3]
MODULE_PATH = ROOT / ".github" / "scripts" / "upstream_watch.py"
WORKFLOW_PATH = ROOT / ".github" / "workflows" / "upstream-watch.yml"


def load_module():
    spec = importlib.util.spec_from_file_location("inputcodex_upstream_watch", MODULE_PATH)
    if spec is None or spec.loader is None:
        raise RuntimeError("无法加载 upstream_watch.py")
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


watch = load_module()


def baseline():
    return watch.Baseline(
        upstream_repository="BigPizzaV3/CodexPlusPlus",
        release_tag="v1.2.41",
        release_published_at="2026-07-20T01:48:40Z",
        release_url="https://github.com/BigPizzaV3/CodexPlusPlus/releases/tag/v1.2.41",
        release_commit="3dafffcafb2566a1e8bce4b35671656d6adb3eda",
    )


def observation(**overrides):
    values = {
        "observed_at": "2026-07-21T20:00:00Z",
        "latest_release_tag": "v1.2.41",
        "latest_release_published_at": "2026-07-20T01:48:40Z",
        "latest_release_url": "https://github.com/BigPizzaV3/CodexPlusPlus/releases/tag/v1.2.41",
        "latest_release_commit": "3dafffcafb2566a1e8bce4b35671656d6adb3eda",
        "locked_release_tag": "v1.2.41",
        "locked_release_commit": "3dafffcafb2566a1e8bce4b35671656d6adb3eda",
        "main_commit": "6fa0a57decbb3382771a981247e6922799e97f5d",
    }
    values.update(overrides)
    return watch.Observation(**values)


class FakeClient:
    def __init__(self, issues=None):
        self.issues = list(issues or [])
        self.created = []
        self.updated = []
        self.operations = []

    def list_issues(self, repository):
        self.repository = repository
        return list(self.issues)

    def create_issue(self, repository, title, body, labels):
        self.operations.append(("create", body.splitlines()[0]))
        issue = {
            "number": 100 + len(self.created),
            "title": title,
            "body": body,
            "state": "open",
            "html_url": f"https://github.com/{repository}/issues/{100 + len(self.created)}",
        }
        self.created.append({"repository": repository, "title": title, "body": body, "labels": labels})
        self.issues.append(issue)
        return issue

    def update_issue(self, repository, number, title, body, state):
        self.operations.append(("update", body.splitlines()[0]))
        self.updated.append(
            {
                "repository": repository,
                "number": number,
                "title": title,
                "body": body,
                "state": state,
            }
        )
        return {
            "number": number,
            "title": title,
            "body": body,
            "state": state,
            "html_url": f"https://github.com/{repository}/issues/{number}",
        }


class ScriptedGitHubClient(watch.GitHubClient):
    def __init__(self, responses):
        super().__init__("test-token")
        self.responses = list(responses)
        self.paths = []

    def get(self, path):
        self.paths.append(path)
        if not self.responses:
            raise AssertionError(f"没有为请求准备响应：{path}")
        return self.responses.pop(0)


class FailingAlertClient(FakeClient):
    def create_issue(self, repository, title, body, labels):
        if body.splitlines()[0] != watch.STATE_MARKER:
            raise watch.MonitorError("模拟告警写入失败")
        return super().create_issue(repository, title, body, labels)


class DecisionTests(unittest.TestCase):
    def test_initial_same_release_does_not_raise_alert(self):
        self.assertEqual(watch.decide_events(baseline(), None, observation()), [])

    def test_initial_main_is_only_recorded_as_baseline(self):
        changed_main = observation(main_commit="7" * 40)
        self.assertNotIn("main-change", [event.kind for event in watch.decide_events(baseline(), None, changed_main)])

    def test_new_release_is_detected_once(self):
        current = observation(
            latest_release_tag="v1.2.42",
            latest_release_published_at="2026-07-21T01:00:00Z",
            latest_release_url="https://github.com/BigPizzaV3/CodexPlusPlus/releases/tag/v1.2.42",
            latest_release_commit="4" * 40,
        )
        events = watch.decide_events(baseline(), observation(), current)
        self.assertEqual([event.kind for event in events], ["new-release"])

    def test_locked_release_tag_drift_is_independent_of_latest_release(self):
        current = observation(locked_release_commit="5" * 40)
        events = watch.decide_events(baseline(), observation(), current)
        self.assertEqual([event.kind for event in events], ["release-tag-drift"])

    def test_release_metadata_change_is_detected_for_same_tag(self):
        current = observation(latest_release_published_at="2026-07-20T02:00:00Z")
        events = watch.decide_events(baseline(), observation(), current)
        self.assertEqual([event.kind for event in events], ["release-metadata-change"])

    def test_main_change_requires_previous_successful_state(self):
        previous = observation()
        current = observation(main_commit="6" * 40)
        events = watch.decide_events(baseline(), previous, current)
        self.assertEqual([event.kind for event in events], ["main-change"])

    def test_event_fingerprint_ignores_scan_time(self):
        first = watch.decide_events(
            baseline(),
            observation(),
            observation(main_commit="6" * 40, observed_at="2026-07-21T20:00:00Z"),
        )[0]
        second = watch.decide_events(
            baseline(),
            observation(),
            observation(main_commit="6" * 40, observed_at="2026-07-21T21:00:00Z"),
        )[0]
        self.assertEqual(first.fingerprint, second.fingerprint)


class StateTests(unittest.TestCase):
    def test_state_round_trip(self):
        current = observation()
        body = watch.render_state_body(current)
        self.assertEqual(watch.parse_state_body(body), current)

    def test_material_state_ignores_observed_time(self):
        first = observation(observed_at="2026-07-21T20:00:00Z")
        second = observation(observed_at="2026-07-21T21:00:00Z")
        self.assertTrue(watch.same_material_observation(first, second))

    def test_state_body_contains_exact_machine_marker_first(self):
        body = watch.render_state_body(observation())
        self.assertEqual(body.splitlines()[0], watch.STATE_MARKER)


class IssueSafetyTests(unittest.TestCase):
    def test_non_machine_issue_is_ignored(self):
        issues = [{"number": 1, "title": "人工 Issue", "body": f"引用 {watch.STATE_MARKER}", "state": "open"}]
        self.assertIsNone(watch.find_unique_issue(issues, watch.STATE_MARKER))

    def test_duplicate_exact_machine_markers_fail_closed(self):
        issues = [
            {"number": 1, "title": "状态一", "body": watch.STATE_MARKER, "state": "open"},
            {"number": 2, "title": "状态二", "body": watch.STATE_MARKER, "state": "open"},
        ]
        with self.assertRaises(watch.MonitorError):
            watch.find_unique_issue(issues, watch.STATE_MARKER)

    def test_same_fingerprint_has_no_write(self):
        fingerprint = "a" * 64
        body = f"{watch.ALERT_MARKERS['main-change']}\n{watch.fingerprint_marker(fingerprint)}\n"
        client = FakeClient(
            [
                {
                    "number": 9,
                    "title": "既有预警",
                    "body": body,
                    "state": "open",
                    "html_url": "https://github.com/nonononull/inputcodex/issues/9",
                }
            ]
        )
        result = watch.upsert_machine_issue(
            client=client,
            repository="nonononull/inputcodex",
            issues=client.list_issues("nonononull/inputcodex"),
            marker=watch.ALERT_MARKERS["main-change"],
            fingerprint=fingerprint,
            title="新标题不应触发写入",
            body="新正文不应触发写入",
        )
        self.assertEqual(result.action, "unchanged")
        self.assertEqual(client.created, [])
        self.assertEqual(client.updated, [])

    def test_changed_fingerprint_updates_only_exact_marker_issue(self):
        marker = watch.ALERT_MARKERS["main-change"]
        existing = {
            "number": 9,
            "title": "旧预警",
            "body": f"{marker}\n{watch.fingerprint_marker('a' * 64)}\n",
            "state": "closed",
            "html_url": "https://github.com/nonononull/inputcodex/issues/9",
        }
        client = FakeClient([existing])
        result = watch.upsert_machine_issue(
            client=client,
            repository="nonononull/inputcodex",
            issues=client.list_issues("nonononull/inputcodex"),
            marker=marker,
            fingerprint="b" * 64,
            title="新预警",
            body="新正文",
        )
        self.assertEqual(result.action, "updated")
        self.assertEqual(client.updated[0]["number"], 9)
        self.assertEqual(client.updated[0]["state"], "open")


class ExecutionTests(unittest.TestCase):
    def test_initial_run_creates_only_state_issue(self):
        client = FakeClient()
        summary = watch.execute_monitor(
            client=client,
            target_repository="nonononull/inputcodex",
            baseline=baseline(),
            current=observation(),
        )
        self.assertEqual(summary.event_count, 0)
        self.assertEqual(summary.state_action, "created")
        self.assertEqual([item["body"].splitlines()[0] for item in client.created], [watch.STATE_MARKER])

    def test_repeated_same_observation_has_no_write(self):
        state_issue = {
            "number": 7,
            "title": "状态",
            "body": watch.render_state_body(observation()),
            "state": "open",
            "html_url": "https://github.com/nonononull/inputcodex/issues/7",
        }
        client = FakeClient([state_issue])
        summary = watch.execute_monitor(
            client=client,
            target_repository="nonononull/inputcodex",
            baseline=baseline(),
            current=observation(observed_at="2026-07-21T21:00:00Z"),
        )
        self.assertEqual(summary.state_action, "unchanged")
        self.assertEqual(client.operations, [])

    def test_closed_state_issue_is_reopened_even_without_material_change(self):
        state_issue = {
            "number": 7,
            "title": "状态",
            "body": watch.render_state_body(observation()),
            "state": "closed",
            "html_url": "https://github.com/nonononull/inputcodex/issues/7",
        }
        client = FakeClient([state_issue])
        summary = watch.execute_monitor(
            client=client,
            target_repository="nonononull/inputcodex",
            baseline=baseline(),
            current=observation(observed_at="2026-07-21T21:00:00Z"),
        )
        self.assertEqual(summary.state_action, "updated")
        self.assertEqual(client.updated[0]["state"], "open")

    def test_alert_is_written_before_state_advances(self):
        previous = observation()
        state_issue = {
            "number": 7,
            "title": "状态",
            "body": watch.render_state_body(previous),
            "state": "open",
            "html_url": "https://github.com/nonononull/inputcodex/issues/7",
        }
        client = FakeClient([state_issue])
        watch.execute_monitor(
            client=client,
            target_repository="nonononull/inputcodex",
            baseline=baseline(),
            current=observation(main_commit="6" * 40),
        )
        self.assertEqual(
            client.operations,
            [("create", watch.ALERT_MARKERS["main-change"]), ("update", watch.STATE_MARKER)],
        )

    def test_failed_alert_does_not_advance_state(self):
        previous = observation()
        state_issue = {
            "number": 7,
            "title": "状态",
            "body": watch.render_state_body(previous),
            "state": "open",
            "html_url": "https://github.com/nonononull/inputcodex/issues/7",
        }
        client = FailingAlertClient([state_issue])
        with self.assertRaises(watch.MonitorError):
            watch.execute_monitor(
                client=client,
                target_repository="nonononull/inputcodex",
                baseline=baseline(),
                current=observation(main_commit="6" * 40),
            )
        self.assertEqual(client.updated, [])


class WorkflowContractTests(unittest.TestCase):
    def test_workflow_has_runtime_triggers_and_read_only_pr_validation(self):
        text = WORKFLOW_PATH.read_text(encoding="utf-8")
        self.assertRegex(text, r"(?m)^  pull_request:$")
        self.assertRegex(text, r"(?m)^  schedule:$")
        self.assertRegex(text, r"(?m)^    - cron: '17 \*/6 \* \* \*'$")
        self.assertRegex(text, r"(?m)^  workflow_dispatch:$")
        self.assertNotRegex(text, r"(?m)^  push:$")
        self.assertRegex(text, r"(?m)^permissions:\n  contents: read$")
        self.assertRegex(text, r"(?m)^env:\n  PYTHONPYCACHEPREFIX: /tmp/inputcodex-pycache$")
        self.assertEqual(text.count("issues: write"), 1)
        self.assertRegex(text, r"(?ms)^  watch:\n.*?^    permissions:\n      contents: read\n      issues: write$")
        self.assertRegex(text, r"(?m)^    if: github\.event_name != 'pull_request'$")
        self.assertRegex(text, r"(?m)^    runs-on: ubuntu-latest$")
        self.assertRegex(text, r"(?m)^    timeout-minutes: 10$")
        self.assertIn("group: upstream-monitor-", text)
        self.assertIn("cancel-in-progress: true", text)

    def test_workflow_pins_official_checkout_and_has_no_forbidden_surface(self):
        text = WORKFLOW_PATH.read_text(encoding="utf-8")
        self.assertRegex(text, r"actions/checkout@[0-9a-f]{40}")
        self.assertEqual(text.count("persist-credentials: false"), 2)
        forbidden = ["pull_request_target", "self-hosted", "upload-artifact", "cargo ", "target/"]
        for value in forbidden:
            self.assertNotIn(value, text)

    def test_workflow_executes_repository_script_without_dynamic_shell_input(self):
        text = WORKFLOW_PATH.read_text(encoding="utf-8")
        self.assertIn("python3 .github/scripts/upstream_watch.py", text)
        self.assertIn("python3 -m unittest discover", text)
        self.assertNotRegex(text, re.compile(r"\$\{\{\s*github\.event\.(issue|comment)"))
        self.assertNotRegex(
            text,
            re.compile(r"\$\{\{\s*github\.event\.pull_request\.(?!number(?:\s|\}|\|))"),
        )


class SourceLockTests(unittest.TestCase):
    def test_repository_source_lock_loads_as_baseline(self):
        loaded = watch.load_baseline(ROOT / "upstream" / "source-lock.json")
        self.assertEqual(loaded, baseline())

    def test_invalid_source_lock_fails_closed(self):
        with self.assertRaises(watch.MonitorError):
            watch.baseline_from_data({"snapshot": {"repository": "错误仓库"}})


class GitHubAdapterTests(unittest.TestCase):
    def test_issue_api_rejects_non_object_entries(self):
        client = ScriptedGitHubClient([[{"number": 1}, "损坏条目"]])
        with self.assertRaises(watch.MonitorError):
            client.list_issues("nonononull/inputcodex")

    def test_issue_api_filters_pull_requests_after_validating_entries(self):
        client = ScriptedGitHubClient(
            [[{"number": 1, "body": ""}, {"number": 2, "pull_request": {}, "body": ""}]]
        )
        self.assertEqual(client.list_issues("nonononull/inputcodex"), [{"number": 1, "body": ""}])

    def test_fetch_observation_resolves_release_locked_tag_and_main(self):
        client = ScriptedGitHubClient(
            [
                {
                    "tag_name": "v1.2.41",
                    "published_at": "2026-07-20T01:48:40Z",
                    "html_url": "https://github.com/BigPizzaV3/CodexPlusPlus/releases/tag/v1.2.41",
                },
                {"object": {"type": "commit", "sha": "3" * 40}},
                {"object": {"type": "commit", "sha": "3" * 40}},
                {"object": {"type": "commit", "sha": "6" * 40}},
            ]
        )
        current = watch.fetch_observation(client, baseline())
        self.assertEqual(current.latest_release_commit, "3" * 40)
        self.assertEqual(current.locked_release_commit, "3" * 40)
        self.assertEqual(current.main_commit, "6" * 40)
        self.assertEqual(
            client.paths,
            [
                "/repos/BigPizzaV3/CodexPlusPlus/releases/latest",
                "/repos/BigPizzaV3/CodexPlusPlus/git/ref/tags/v1.2.41",
                "/repos/BigPizzaV3/CodexPlusPlus/git/ref/tags/v1.2.41",
                "/repos/BigPizzaV3/CodexPlusPlus/git/ref/heads/main",
            ],
        )

    def test_safe_error_text_removes_token_and_newlines(self):
        text = watch.safe_error_text(RuntimeError("第一行\nsecret-token\r第二行"), "secret-token")
        self.assertNotIn("secret-token", text)
        self.assertNotIn("\n", text)
        self.assertNotIn("\r", text)


if __name__ == "__main__":
    unittest.main()
