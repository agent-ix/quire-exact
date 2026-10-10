---
id: FR-369
title: "Carry the precise cause of a checked-invariant failure"
type: FR
relationships:
  - target: ix://agent-ix/quire-exact/FR-090
    type: references
  - target: ix://agent-ix/quire-exact/FR-096
    type: references
  - target: ix://agent-ix/quire-exact/FR-089
    type: references
---
# FR-369: Carry the precise cause of a checked-invariant failure

## Description

When the exact kernel, the checked-expression residue in `quire-contract-runtime`, the shared `quire-semantic-value` leaf, or a generated CG oracle detects a checked-program invariant failure, the shared `quire-exact::Refusal` SHALL carry the failure's closed, typed cause. A caller SHALL distinguish a depleted call-depth budget from a re-entrant meter borrow, and each other condition in the tables below, by matching that cause without inspecting a message or guessing from a location. These failures remain internal faults, not catalog refusals or proof results.

## Inputs

- A checked operation or checked-expression call using the shared `Refusal`.
- The condition detected at one of the kernel, runtime-residue or generated-oracle sites below.

## Outputs

- `Refusal::CheckedInvariant { cause: CheckedInvariantCause }`, carried through
  the existing `Outcome::Refused` or `Result::Err` path.

## Behavior

The public `CheckedInvariantCause` SHALL be a closed enum of the thirty-one variants in the two tables. `ScheduledComparisonRefused`, `EqualityQuantityConversionRejected`, `GeneratedTypeCheckRejected` and `GeneratedEqualityCheckRejected` each SHALL carry `{ cause: IllTypedCause }`; the other twenty-seven variants are unit variants. The existing typed refusal returned by a scheduled comparator, an equality quantity conversion, or CG's RT type/equality check SHALL be retained in that payload without string conversion. `IllTypedCause` is already owned and exported by `quire-exact::comparison`, which RT origin/main imports from the kernel; no kernel-to-RT or kernel-to-QSV dependency or duplicate type is introduced. The enum SHALL support `Copy`, `Clone`, `Debug`, `Eq`, `Hash` and `PartialEq` so a caller can compare and retain a cause without retaining a value or allocating a message. The existing `Refusal` remains `Clone`, `Debug`, `Eq` and `PartialEq`.

| Cause | Trigger and owning source on the measured main revisions |
|---|---|
| `CollectionElementNotAdmitted` | An evaluated element is outside its declared collection element type: kernel `src/collection.rs:285-286`; RT `src/exact/collection.rs:136-137`. |
| `DeferredResultNotAdmitted` | A deferred composite field/result is outside its declared value type: kernel `src/value.rs:1059-1064`; RT `src/exact/composite.rs:1507-1512`; QSV `src/declaration.rs:2141-2146`. |
| `CanonicalKeyUnavailable` | Collection ordering cannot obtain the canonical key of a member: kernel `src/collection.rs:487-496`; RT `src/exact/collection.rs:301-310`. |
| `BoundedDivisionExpected` | The bounded division member path receives `IntegerDomain::Mathematical`: kernel `src/division.rs:121-128`. |
| `CollectionKindMismatch` | Equality reaches two collection values with different collection kinds: kernel `src/equality.rs:178-181`; RT `src/exact/equality.rs:308-311`. |
| `PopulationPair` | Kernel equality receives two population values, which the leaf cannot resolve: kernel `src/equality.rs:192-207`; [FR-089](./FR-089-kernel-refuses-a-population-pair.md). A population paired with another value kind uses `ValueKindMismatch`. |
| `ValueKindMismatch` | Equality reaches incompatible value variants other than the two-collection kind mismatch or two-population case: kernel `src/equality.rs:192-207`; RT `src/exact/equality.rs:324-338`. |
| `CallDepthExceeded` | Any of RT's three shared checked-package entry paths fails `enter()` at its configured depth: `src/exact/expression.rs:796,852,919`. |
| `UnknownCheckedFunction` | RT `run_call` or `Frame::call` finds no named function after checking: `src/exact/expression.rs:802,922`. |
| `ForeignCheckedExpression` | RT `plan_evaluation` finds an expression checked against another package: `src/exact/expression.rs:829-834`. |
| `MeterBorrowConflict` | RT `Frame::call` or `Frame::meter` cannot acquire its shared mutable meter borrow: `src/exact/expression.rs:925-934,956-960`. |
| `EqualityScheduleMismatch` | RT or QSV's checked text/enum/quantity schedule receives values of another shape: RT `src/exact/equality.rs:167-185`; QSV `src/declaration.rs:2412-2416`. |
| `ScheduledComparisonRefused { cause }` | RT or QSV's selected text/enum/quantity comparator returns `IllTyped { cause }` after a successful checked schedule: RT `src/exact/equality.rs:167-192`; QSV `src/declaration.rs:2418`; the kernel-owned `IllTypedCause` is retained. |
| `EqualityOperandSourceNotAdmitted` | A completed equality operand fails its checked source type's admission: RT `src/exact/equality.rs:432-433`; QSV `src/declaration.rs:2532-2537`. |
| `EqualityOperandNonIntegralDecimal` | The checked equality conversion to an integer receives a decimal whose normalized rational is not integral: RT `src/exact/equality.rs:464-467`; QSV `src/declaration.rs:2571-2574`. |
| `EqualityQuantityConversionRejected { cause }` | A checked equality quantity conversion returns `IllTyped { cause }`: RT `src/exact/equality.rs:471-473`; QSV `src/declaration.rs:2582-2584`. Retain the original kernel-owned `IllTypedCause`. |
| `EqualityQuantityNonExactPlacement` | Equality requested an exact quantity conversion but the conversion returned decimal or integer placement: RT `src/exact/equality.rs:475-480`; QSV `src/declaration.rs:2586-2591`. |
| `EqualityConversionShapeMismatch` | A checked equality conversion reaches no matching source/target/value arm: RT `src/exact/equality.rs:484`; QSV `src/declaration.rs:2595`. |
| `EqualityOperandTargetNotAdmitted` | The converted comparison value fails its checked target type's admission: RT `src/exact/equality.rs:486-489`; QSV `src/declaration.rs:2597-2600`. |
| `EqualityUnitUnresolved` | QSV cannot resolve a checked quantity's unit either for the selected quantity comparison or for a checked equality conversion: `src/declaration.rs:2402-2404,2578-2580`. |
| `EqualityEnumVariantUnresolved` | QSV cannot resolve a checked enum operand's variant in the comparison's captured declaration: `src/declaration.rs:2393-2398`. |
| `ExpectedIntegerPlacement` | A scale-zero integer quantity placement yields a retained value that is not an integer: RT `src/exact/quantity.rs:541-546`; QSV `src/quantity.rs:795-800`. |

The following distinct causes cover CG's oracle constructors (generated source
and the pre-check declaration placeholder) on
quire-contract-codegen main `1b225f4cf25450ee662fa97c91286b2a76f64ff8`.
They name shared checked-oracle failures, not CG package or model types.

| Cause | Trigger and owning CG source |
|---|---|
| `GeneratedBodyPlaceholderInvoked` | A function declaration's pre-check placeholder body runs, which should be replaced before execution: `src/oracle/function/mod.rs:1045`. |
| `GeneratedArgumentShapeMismatch` | A generated scalar or equality function body receives the wrong number or shape of arguments: `src/oracle/function/mod.rs:1531,1551`. |
| `GeneratedOperandKindUnsupported` | A generated equality body receives a left operand outside its classified Boolean/Integer kinds: `src/oracle/function/mod.rs:1555`. |
| `GeneratedUnexpectedOutcome` | A generated Boolean, scalar or equality body's expected outcome adapter sees another outcome arm: `src/oracle/boolean_v1.rs:846`, `src/oracle/function/mod.rs:1539,1571`. |
| `GeneratedIntervalInvalid` | A generated native integer bound cannot form a nonempty interval: `src/oracle/boolean_v1.rs:855`. |
| `GeneratedEnvironmentRejected` | A generated Boolean/equality body cannot form its RT type environment: `src/oracle/boolean_v1.rs:872`, `src/oracle/function/mod.rs:1559`. |
| `GeneratedTypeCheckRejected { cause }` | A generated equality oracle's declared comparison type fails RT's `check_type`; retain its kernel-owned `IllTypedCause`: `src/oracle/equality/mod.rs:1535,1546`. |
| `GeneratedEqualityCheckRejected { cause }` | A generated Boolean/equality body or oracle fails RT's `check_equality`; retain its kernel-owned `IllTypedCause`: `src/oracle/boolean_v1.rs:880`, `src/oracle/equality/mod.rs:1555`, `src/oracle/function/mod.rs:1563`. |
| `GeneratedDescriptorReconstructionFailed` | A generated equality oracle cannot reconstruct one of its authored source/target types: `src/oracle/equality/mod.rs:1527,1530,1538,1541`. |

The two tables classify all production constructors on quire-exact main
`497c581c9dd14eb70083488209827dd8a5b3e03a`,
quire-contract-runtime main `4180f0ea135767637b10a981393c5aef22220ace`,
quire-semantic-value `e0ada80708fe73469923a9d6c2263cfebc0087a0`,
and the CG revision above. QSV's `src/declaration.rs:2146` uses
`DeferredResultNotAdmitted`. At each call site of its shared `invariant()`
helper at line 2422, QSV SHALL select the corresponding typed cause: enum
lookup, unit lookup, schedule shape and comparator refusal have the rows above;
the seven equality-operand call sites
use the six RT/QSV rows plus `EqualityUnitUnresolved`. Its
`src/quantity.rs:800` uses `ExpectedIntegerPlacement`. The QSV operational
contracts are QSpec FR-143 for deferred record/tuple admission, FR-149 for
checked equality and conversion, and FR-142 for quantity conversion; its
local FR-060/FR-106/FR-107 do not replace those contracts. An admitted
equality conversion is checked from declared bounds before execution, so
a later conversion-shape, admission, unresolved-unit, nonintegral or
unexpected-placement failure is a checked-program internal fault. A reached
`IllTyped` from the selected comparison or quantity conversion remains an
internal fault with its precise `IllTypedCause`, rather than becoming a
catalog `ill_typed` refusal after checking.

These tables do not classify the remaining QSL constructors: QSL
`qsl-eval/src/value/expression/evaluate.rs:238` uses one helper for missing
stack/frame/slot values and distinct checked-node, operator, profile,
composite, query and navigation assumptions.
`qsl-semantics/src/value/model_query.rs:119` uses another helper for an empty
object-identity bridge and wrong reference/collection/option shapes. In
particular, the empty-identity premise is not fully enforced at population
binding admission, so this bridge failure cannot be dismissed as unreachable.
QSL SHALL split or reroute those producers under a QSL-owned specification
before pinning this required-payload API;
assigning all of them one generic default kernel cause is forbidden. The
existing QSL mapping of *received* checked-invariant refusals remains as below.
The kernel's equality catch-all SHALL check the two-population case before
returning `ValueKindMismatch`; otherwise [FR-089](./FR-089-kernel-refuses-a-population-pair.md)
would lose its distinct cause. A constructor added later SHALL choose a
specified cause or extend this closed table and its tests before merging.

For **every** `CheckedInvariant` cause, `Refusal::code()` and
`Refusal::cause()` SHALL return `None`. The typed cause SHALL remain available
to Rust callers through the variant payload and `Refusal` equality; neither
method SHALL synthesize a catalog spelling from it. QSpec FR-096 and FR-100
and QSL's FR-096/FR-100 mapping SHALL treat every such *received* payload as
an internal fault, not an ordinary refusal record. At S6a,
`Machine::stopped` SHALL return `Err(InternalFault)` with code
`runtime_invariant`, stage `S6a` and invariant `checked-program-invariant`
for either located or unlocated stops; it returns no `Evaluation` holding the
checked-invariant refusal. The public call path SHALL carry that fault as
`CallFailure::Fault`. A replay fault SHALL settle as `Failed`, category
internal failure, basis unavailable; an ordinary replay refusal remains
`Inconclusive`. A fault is neither a proof nor a certification rejection.
`CancelCause::{Requested, Deadline}` remains a separate incomplete outcome.
An unknown received cause SHALL retain fault classification and provenance;
it cannot fall through to a catalog refusal, `Completed`, a clause verdict,
or a verified proof. CG's generated oracles SHALL construct the exact typed payload,
and downstream replay adapters SHALL preserve it or match it explicitly without
discarding its cause. They SHALL preserve their existing charge and
failure behavior; they SHALL NOT use a default cause, a string message, or a
compatibility path for the former unit variant. A generated pre-charge failure
remains pre-charge; the payload does not authorize extra work.

This shared carrier is limited to the kernel's checked failures, RT's
currently extracted exact residue, QSV's shared semantic-value failures and
CG's generated checked-oracle failures listed above. It does not import a QSL
model, checker, catalog, or evaluator type, and does not admit an arbitrary
caller-supplied reason.
[FR-090](./FR-090-kernel-outcome-carries-no-caller-cause.md)
states this narrow boundary.

## Acceptance Criteria

| ID | Criteria | Verification |
|---|---|---|
| FR-369-AC-1 | The public `Refusal::CheckedInvariant` has a required `CheckedInvariantCause` payload whose closed variant set and four typed `IllTypedCause` payloads equal the thirty-one table rows; `CheckedInvariantCause` implements the stated traits, and the old unit constructor does not compile. | Test |
| FR-369-AC-2 | Every kernel producer in collection, value, division and equality returns exactly its table cause; two population values return `PopulationPair`, a population paired with another kind returns `ValueKindMismatch`, and unlike collection kinds return `CollectionKindMismatch`. | Test |
| FR-369-AC-3 | Every RT checked-package/function producer returns its table cause: depth exhaustion on all three entry paths is `CallDepthExceeded`, unknown name is `UnknownCheckedFunction`, a foreign expression is `ForeignCheckedExpression`, and each meter reborrow is `MeterBorrowConflict`. | Test |
| FR-369-AC-4 | Every RT collection, composite, equality and quantity producer returns its table cause, including all six `operand_value` failures and the original kernel-owned `IllTypedCause` when a scheduled comparison or checked equality quantity conversion refuses; no site flattens a comparison or conversion error to a message. | Test |
| FR-369-AC-5 | For every typed checked-invariant cause, `Refusal::code()` and `Refusal::cause()` return `None`, and `Eq` distinguishes otherwise equal refusals whose causes differ. Ordinary kernel refusal codes and causes are unchanged. | Test |
| FR-369-AC-6 | Every received typed cause follows QSL's existing internal-fault path: no refusal record or `Evaluation`, `runtime_invariant`/`S6a`/`checked-program-invariant` at S6a, `CallFailure::Fault` at the public call, and `Failed` with unavailable basis for an internal replay fault. It never becomes an ordinary replay refusal, `Inconclusive`, `Incomplete(Cancelled)`, a proved result or a certification rejection; an unknown received cause retains fault provenance. | Inspection |
| FR-369-AC-7 | CG's native Boolean, equality, function and pre-check placeholder constructors use the exact nine CG-owned causes in the table, retaining `IllTypedCause` from `check_type` and `check_equality`; downstream replay consumers preserve the typed payload or match it explicitly, retain charge/failure behavior, and accept neither the former unit constructor nor an invented default. | Test |
| FR-369-AC-8 | The public carrier contains no QSL model/checker/catalog type or dependency on RT or QSV, RT and QSV own no second refusal enum or compatibility shim, and an exhaustive source inventory finds every kernel, RT, QSV and CG production checked-invariant constructor assigned exactly one cause from the two tables. | Inspection |
| FR-369-AC-9 | QSV's deferred field or tuple result uses `DeferredResultNotAdmitted`; missing enum variant and unresolved unit use their distinct rows; schedule mismatch and comparator refusal retain their existing rows; each equality operand failure uses its exact table row; and scale-zero integer placement uses `ExpectedIntegerPlacement`. The comparator and quantity conversion retain their original `IllTypedCause` payload, while prior non-result and charge behavior is unchanged. | Test |

## Status

Planned. IR-707 owns the kernel carrier and kernel producer migration; IR-708
owns RT producers, IR-712 owns QSV producers, IR-709 owns CG producers and
consumers, and QSL's direct constructor migration is required before it pins
the new kernel API. No runtime behavior is claimed by this spec-only change.

## Dependencies

- Kernel [FR-090](./FR-090-kernel-outcome-carries-no-caller-cause.md),
  [FR-096](./FR-096-kernel-refusal-code-and-cause.md) and
  [FR-089](./FR-089-kernel-refuses-a-population-pair.md) are amended with this
  carrier's ownership, `None` mapping and population-specific cause.
- [TC-917](../test-cases/TC-917-typed-checked-invariant-causes.md) covers the
  kernel mapping and public shape. RT FR-273/TC-194 must test depth, unknown
  name, foreign expression and meter borrow through the shared type; RT's
  collection/equality/composite/quantity cases need matching tagged tests.
  QSV's QSpec FR-142/FR-143/FR-149 contracts must test their corresponding
  internal-fault routes without adding a QSV-local duplicate requirement.
  CG FR-018/FR-021 must migrate generated constructors and replay consumers;
  QSpec/QSL FR-096/FR-100 must migrate unit matches. QSL's two production
  constructor sites additionally need a QSL-owned typed-cause/family-fault
  ruling *before* the QSL lock pins the kernel API. These are downstream
  code/spec changes, not code claims of this spec-only slice.
