#!/usr/bin/env python3
"""
Poppet Codebase Line-of-Code and Language Distribution Analyzer.

Scans all source directories and reports precise line, blank, comment,
and code counts across Befunge-98, Rust, Rhai, C, and Python.

Copyright (c) 2026 Aneesh Krishna Parthasarathy
Licensed under the ISC License.
"""

import os

EXCLUDE_DIRS = {".git", "target", "bin", "build", "__pycache__", ".zig-cache"}

LANG_MAP = {
    ".b98": "Befunge-98",
    ".bf": "Befunge-98",
    ".rs": "Rust",
    ".rhai": "Rhai",
    ".c": "C",
    ".h": "C Header",
    ".py": "Python",
    ".md": "Markdown",
}

def analyze_file(filepath):
    lines = 0
    blank = 0
    comment = 0
    code = 0
    ext = os.path.splitext(filepath)[1].lower()

    with open(filepath, "r", encoding="utf-8", errors="replace") as f:
        for line in f:
            lines += 1
            trimmed = line.strip()
            if not trimmed:
                blank += 1
            elif ext in {".b98", ".bf"} and trimmed.startswith(";"):
                comment += 1
            elif ext in {".rs", ".rhai", ".c", ".h"} and trimmed.startswith("//"):
                comment += 1
            elif ext == ".py" and trimmed.startswith("#"):
                comment += 1
            else:
                code += 1
    return lines, blank, comment, code

def main():
    root_dir = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))
    stats = {}

    for root, dirs, files in os.walk(root_dir):
        dirs[:] = [d for d in dirs if d not in EXCLUDE_DIRS]
        for fname in files:
            ext = os.path.splitext(fname)[1].lower()
            if ext in LANG_MAP:
                lang = LANG_MAP[ext]
                fpath = os.path.join(root, fname)
                tot, blk, cmt, cd = analyze_file(fpath)
                if lang not in stats:
                    stats[lang] = {"files": 0, "total": 0, "blank": 0, "comment": 0, "code": 0}
                stats[lang]["files"] += 1
                stats[lang]["total"] += tot
                stats[lang]["blank"] += blk
                stats[lang]["comment"] += cmt
                stats[lang]["code"] += cd

    print("=" * 80)
    print("  POPPET CODEBASE SCALE & LANGUAGE DISTRIBUTION")
    print("=" * 80)
    print(f"{'Language':<16} {'Files':>6} {'Total Lines':>12} {'Code Lines':>12} {'Comments':>10} {'Blank':>8}")
    print("-" * 80)

    grand_files = 0
    grand_total = 0
    grand_code = 0
    grand_comment = 0
    grand_blank = 0

    # Sort languages so Befunge is displayed first
    sorted_langs = sorted(stats.keys(), key=lambda l: (0 if "Befunge" in l else 1, l))

    for lang in sorted_langs:
        d = stats[lang]
        grand_files += d["files"]
        grand_total += d["total"]
        grand_code += d["code"]
        grand_comment += d["comment"]
        grand_blank += d["blank"]
        print(f"{lang:<16} {d['files']:>6d} {d['total']:>12d} {d['code']:>12d} {d['comment']:>10d} {d['blank']:>8d}")

    print("=" * 80)
    print(f"{'Total':<16} {grand_files:>6d} {grand_total:>12d} {grand_code:>12d} {grand_comment:>10d} {grand_blank:>8d}")
    print("=" * 80)

if __name__ == "__main__":
    main()
