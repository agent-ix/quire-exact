# SPDX-License-Identifier: AGPL-3.0-or-later
# Copyright (C) 2026 Agent-IX
"""quire-exact's documentation figures, rendered by the docs-figures skill.

Regenerate from the repository root with the agent-ix/dev-tools skill:

    python3 -m pip install fonttools
    python3 <dev-tools>/skills/docs-figures/scripts/render.py docs/images/figures.py

Add --check to confirm the committed SVGs are current. The outcome variants are
the ones in `src/outcome.rs`; the arithmetic examples are drawn by hand.
"""

from docs_figures import FONTS, SMALL, TITLE, Figure  # noqa: F401

STYLE = "ix-docs"

# One role per kind of thing, fixed across the Quire repositories.
ROLES = [
    ("code", "authored source", "box"),
    ("logic", "compiler stage", "box"),
    ("model", "checked package / IR", "box"),
    ("evidence", "outcome", "box"),
    ("ink", "input", "box"),
]



# ---- figures -----------------------------------------------------------------


def hero():
    """README hero: float rounding versus exact rationals versus an honest no."""
    rows = [
        ("0.1 + 0.2", "binary float", "rounds every step", "0.30000000000000004", "close, but not 0.3", "logic", "evidence"),
        ("1/10 + 2/10", "exact rational", "fraction in lowest terms", "3/10", "exactly 0.3", "model", "evidence"),
        ("1/2 ÷ 0", "exact rational", "no value exists", "undefined", "division by zero", "model", "evidence"),
    ]
    f = Figure(
        "hero",
        980,
        264,
        "Binary floating point rounds 0.1 + 0.2 to 0.30000000000000004. quire-exact adds the "
        "fractions 1/10 and 2/10 exactly and gets 3/10, and when an operation such as 1/2 divided "
        "by 0 has no value it returns undefined.",
    )
    for i, (expr, how, why, out, note, mid, end) in enumerate(rows):
        y = i * 80
        f.box(0, y, 200, 56, "ink", expr, "input")
        f.line([(200, y + 28), (297, y + 28)], "ink")
        f.box(300, y, 260, 56, mid, how, why)
        f.line([(560, y + 28), (657, y + 28)], mid, weight=2.4)
        f.box(660, y, 320, 56, end, out, note)
    f.legend(0, 258, [("ink", "input", "box"), ("logic", "rounding arithmetic", "box"),
                      ("model", "exact arithmetic", "box"), ("evidence", "answer", "box")])
    return f


def outcomes():
    """The four ways an exact operation ends."""
    rows = [
        ("Completed", "a value", "the exact result, no rounding", "model"),
        ("Undefined", "no value exists", "division by zero · NaN · empty reduction", "evidence"),
        ("Refused", "value not admitted", "outside the declared integer or decimal range", "evidence"),
        ("Incomplete", "budget ran out", "a named limit stopped it · no partial value", "evidence"),
    ]
    f = Figure(
        "outcomes",
        980,
        232,
        "Every exact operation returns exactly one Outcome: Completed with a value, Undefined when "
        "no value exists, Refused when the value is not admitted by the declared type, or "
        "Incomplete when a metered limit stopped it. There is no partial value.",
    )
    f.box(0, 84, 170, 56, "logic", "an operation", "exact arithmetic")
    f.line([(170, 112), (190, 112)], "logic", arrow=False)
    f.line([(190, 28), (190, 196)], "logic", arrow=False)
    for i, (name, short, why, role) in enumerate(rows):
        y = i * 56
        f.line([(190, y + 28), (237, y + 28)], role)
        f.box(240, y, 170, 50, role, name)
        f.text(440, y + 22, short, role, "title")
        f.text(440, y + 40, why, "muted", "mono", SMALL)
    return f


REPOS = [
    (1, 0, "quire-driver", "the quire command", True),
    (2, 0, "quire-integration", "tests the whole chain", True),
    (3, 0, "quire-verification", "plans verification", True),
    (0, 1, "quire-specification", "the standard", True),
    (1, 1, "quire-spec-language", "compiler · evaluator", False),
    (2, 1, "quire-contract-ir", "contract model", False),
    (3, 1, "quire-contract-codegen", "code · tests · proofs", False),
    (4, 1, "quire-contract-runtime", "runs generated checks", False),
    (0, 2, "quire-observation", "qualified observations", False),
    (1, 2, "quire-protocol", "protocol conformance", True),
    (2, 2, "quire-analyze", "SMT consistency", False),
    (0, 3, "quire-mltl", "MLTL over observations", False),
]
FEEDS = [
    ("quire-specification", "quire-spec-language"),
    ("quire-spec-language", "quire-contract-ir"),
    ("quire-contract-ir", "quire-contract-codegen"),
    ("quire-contract-codegen", "quire-contract-runtime"),
    ("quire-spec-language", "quire-protocol"),
    ("quire-contract-ir", "quire-analyze"),
    ("quire-observation", "quire-protocol"),
    ("quire-observation", "quire-mltl"),
]
SHARED = ["quire-exact", "quire-semantic-value", "quire-walk", "quire-canonical",
          "quire-verification-contracts"]


def ecosystem(here="quire-spec-language"):
    """The Quire repositories: the main chain, its branches, and what runs and shares it."""
    f = Figure(
        "ecosystem",
        980,
        476,
        "The Quire repositories. The main chain runs from the standard, quire-specification, to "
        "the compiler, quire-spec-language, then quire-contract-ir, quire-contract-codegen and "
        "quire-contract-runtime. quire-spec-language also feeds quire-protocol, and "
        "quire-contract-ir feeds quire-analyze. quire-observation feeds quire-protocol and "
        "quire-mltl. Above the chain, quire-driver runs it, quire-integration tests it and "
        "quire-verification plans verification. Below it, shared libraries used throughout: "
        "quire-exact, quire-semantic-value, quire-walk, quire-canonical and "
        "quire-verification-contracts.",
    )
    w, h, step = 176, 50, 201
    rows = {0: 20, 1: 120, 2: 210, 3: 290}
    where = {}
    for col, row, name, sub, private in REPOS:
        x, y = col * step, rows[row]
        where[name] = (x, y)
        role = "logic" if name == here else "ink"
        f.box(x, y, w, h, role, name, sub, dashed=private)
    for a, b in FEEDS:
        (ax, ay), (bx, by) = where[a], where[b]
        if ay == by:
            f.line([(ax + w, ay + h / 2), (bx - 3, by + h / 2)], "model")
        else:
            f.line([(ax + w / 2, ay + h), (bx + w / 2, by - 3)], "model")
    f.text(0, 12, "ORCHESTRATION", "muted", "mono", SMALL)
    f.text(0, 112, "THE CHAIN", "muted", "mono", SMALL)
    f.text(0, 382, "SHARED LIBRARIES · USED THROUGHOUT", "muted", "mono", SMALL)
    gap = 10
    widths = [FONTS["title"].width(name, TITLE) + 24 for name in SHARED]
    spare = (980 - sum(widths) - gap * (len(SHARED) - 1)) / len(SHARED)
    x = 0
    for name, w0 in zip(SHARED, widths):
        w0 = round(w0 + spare)
        f.box(x, 390, w0, h, "logic" if name == here else "ink", name)
        x += w0 + gap
    f.legend(0, 472, [("logic", "this repository", "box"), ("ink", "private", "dash"),
                      ("model", "feeds", "fill")])
    return f


def ecosystem_here():
    """The ecosystem figure with this repository highlighted."""
    return ecosystem(here="quire-exact")


FIGURES = [hero, outcomes, ecosystem_here]
