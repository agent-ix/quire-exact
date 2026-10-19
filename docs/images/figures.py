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
        "by 0 has no value it returns undefined instead of a number.",
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
    ("ORCHESTRATION", [("quire-verification", True), ("quire-driver", True), ("quire-integration", True)]),
    ("BACKENDS", [("quire-contract-codegen", False), ("quire-contract-runtime", False),
                  ("quire-analyze", False), ("quire-protocol", True), ("quire-mltl", False)]),
    ("LANGUAGE", [("quire-specification", True), ("quire-spec-language", False), ("quire-contract-ir", False)]),
    ("FOUNDATIONS", [("quire-exact", False), ("quire-semantic-value", False), ("quire-walk", False),
                     ("quire-canonical", False), ("quire-verification-contracts", False),
                     ("quire-observation", False)]),
]


def ecosystem(here="quire-spec-language"):
    """The Quire repositories in bands; each band builds on the ones below."""
    f = Figure(
        "ecosystem",
        980,
        364,
        "The Quire repositories in four bands. Foundations: quire-exact, quire-semantic-value, "
        "quire-walk, quire-canonical, quire-verification-contracts, quire-observation. Language: "
        "quire-specification, quire-spec-language, quire-contract-ir. Backends: "
        "quire-contract-codegen, quire-contract-runtime, quire-analyze, quire-protocol, "
        "quire-mltl. Orchestration: quire-verification, quire-driver, quire-integration.",
    )
    y = 0
    gap = 10
    for band, repos in REPOS:
        f.text(0, y + 12, band, "muted", "mono", SMALL)
        widths = [FONTS["title"].width(name, TITLE) + 24 for name, _ in repos]
        spare = (980 - sum(widths) - gap * (len(repos) - 1)) / len(repos)
        x = 0
        for (name, private), w in zip(repos, widths):
            w = round(w + spare)
            role = "logic" if name == here else "ink"
            f.box(x, y + 20, w, 50, role, name, "private" if private else "public", dashed=private)
            x += w + gap
        y += 88
    f.legend(0, 360, [("logic", "this repository", "box"), ("ink", "private", "dash")])
    return f


def ecosystem_here():
    """The ecosystem figure with this repository highlighted."""
    return ecosystem(here="quire-exact")


FIGURES = [hero, outcomes, ecosystem_here]
