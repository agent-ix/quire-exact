#!/usr/bin/env python3
"""Fail closed while the FR-359-AC-9 proof is disabled."""

import sys


def main():
    print(
        "SKIPPED: DO NOT RUN: exhausts fixed 16 GiB ceiling, returns Inconclusive "
        "MemoryExhausted, proves nothing, blocked work for a week; Peter ordered "
        "it never run until all feature work is done",
        file=sys.stderr,
    )
    return 2


if __name__ == "__main__":
    sys.exit(main())
