#!/usr/bin/env python3
"""Documentation and configuration validation for THE SEED.

Checks, using only reviewed and pinned dependencies:
  * every *.yml / *.yaml file parses as YAML;
  * every *.json file parses as JSON;
  * relative links in Markdown files point to files that exist.

External (http/https/mailto) links and pure #anchors are not checked.
Exit code is non-zero if any problem is found.
"""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path

import yaml

ROOT = Path(__file__).resolve().parents[2]
SKIP_DIRS = {".git", "target", "node_modules", ".venv", "dist", "build"}
LINK_RE = re.compile(r"(?<!!)\[[^\]]*\]\(([^)\s]+)(?:\s+\"[^\"]*\")?\)")
FENCE_RE = re.compile(r"^(```|~~~)")


def iter_files(*suffixes: str):
    for path in sorted(ROOT.rglob("*")):
        if path.is_file() and path.suffix in suffixes:
            if not SKIP_DIRS.intersection(path.relative_to(ROOT).parts):
                yield path


def check_yaml() -> list[str]:
    errors = []
    for path in iter_files(".yml", ".yaml"):
        try:
            with path.open(encoding="utf-8") as fh:
                list(yaml.safe_load_all(fh))
        except yaml.YAMLError as exc:
            errors.append(f"{path.relative_to(ROOT)}: invalid YAML: {exc}")
    return errors


def check_json() -> list[str]:
    errors = []
    for path in iter_files(".json"):
        try:
            json.loads(path.read_text(encoding="utf-8"))
        except json.JSONDecodeError as exc:
            errors.append(f"{path.relative_to(ROOT)}: invalid JSON: {exc}")
    return errors


def check_markdown_links() -> list[str]:
    errors = []
    for path in iter_files(".md"):
        in_fence = False
        for lineno, line in enumerate(path.read_text(encoding="utf-8").splitlines(), 1):
            if FENCE_RE.match(line.strip()):
                in_fence = not in_fence
                continue
            if in_fence:
                continue
            for target in LINK_RE.findall(line):
                if re.match(r"^[a-z][a-z0-9+.-]*:", target, re.I) or target.startswith("#"):
                    continue
                file_part = target.split("#", 1)[0]
                if not file_part:
                    continue
                resolved = (path.parent / file_part).resolve()
                if not resolved.exists():
                    errors.append(
                        f"{path.relative_to(ROOT)}:{lineno}: broken link -> {target}"
                    )
    return errors


def main() -> int:
    errors = check_yaml() + check_json() + check_markdown_links()
    for error in errors:
        print(f"ERROR {error}")
    print(f"validate_docs: {len(errors)} problem(s) found")
    return 1 if errors else 0


if __name__ == "__main__":
    sys.exit(main())
