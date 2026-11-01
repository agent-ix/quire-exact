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
  - target: ix://agent-ix/quire-contract-codegen/FR-042
    type: references
  - target: ix://agent-ix/quire-spec-language/FR-485
    type: references
---
# FR-369: Carry the precise cause of a checked-invariant failure

The shared cause carrier and kernel producers are owned here. The producer controls formerly FR-369-AC-3/4, AC-7 and AC-9 are allocated without weakening their text to [RT FR-278](ix://agent-ix/quire-contract-runtime/FR-278), [CG FR-042](ix://agent-ix/quire-contract-codegen/FR-042) and [QSV FR-122](ix://agent-ix/quire-semantic-value/FR-122), respectively. Their tests remain PLANNED at those owners.

## Description

When the exact kernel, the checked-expression residue in `quire-contract-runtime`, the shared `quire-semantic-value` leaf, or a generated CG oracle detects a checked-program invariant failure, the shared `quire-exact::Refusal` SHALL carry the failure's closed, typed cause. A caller SHALL distinguish each condition in the tables below, including a re-entrant meter borrow and a nested entry into an already-active checked package, by matching that cause without inspecting a message or guessing from a location. These failures remain internal faults, not catalog refusals or proof results. Runtime-managed calls use explicit frames and `work_units` execution fuel under RT FR-273; exhausted fuel returns `Incomplete` with `limit_kind: work_units`, and no call-depth ceiling or `CallDepthExceeded` checked-invariant cause decides an outcome.

## Inputs

- A checked operation or checked-expression call using the shared `Refusal`.
- The condition detected at one of the kernel, runtime-residue or generated-oracle sites below.

## Outputs

- `Refusal::CheckedInvariant { cause: CheckedInvariantCause }`, carried through
  the existing `Outcome::Refused` or `Result::Err` path.

## Behavior

The public `CheckedInvariantCause` SHALL be a closed enum of the thirty-one variants in the two tables. `ScheduledComparisonRefused`, `EqualityQuantityConversionRejected`, `GeneratedTypeCheckRejected` and `GeneratedEqualityCheckRejected` each SHALL carry `{ cause: IllTypedCause }`; the other twenty-seven variants are unit variants. The existing typed refusal returned by a scheduled comparator, an equality quantity conversion, or CG's semantic-value type/equality check SHALL be retained in that payload without string conversion. `IllTypedCause` is already owned and exported by `quire-exact::comparison`, which RT origin/main imports from the kernel; no kernel-to-RT or kernel-to-QSV dependency or duplicate type is introduced. The enum SHALL support `Copy`, `Clone`, `Debug`, `Eq`, `Hash` and `PartialEq` so a caller can compare and retain a cause without retaining a value or allocating a message. The existing `Refusal` remains `Clone`, `Debug`, `Eq` and `PartialEq`.

| Cause | Trigger and owning source on the measured main revisions |
|---|---|
| `NativeIntegerCapacityBreach` | An admitted native integer operation exceeds its certified capacity: kernel `src/native_integer.rs`, `native_arithmetic`. |
| `CollectionElementNotAdmitted` | An evaluated element is outside its declared collection element type: kernel `src/collection.rs:285-286`; RT `src/exact/collection.rs:136-137`. |
| `DeferredResultNotAdmitted` | A deferred composite field/result is outside its declared value type: kernel `src/value.rs:1059-1064`; RT `src/exact/composite.rs:1507-1512`; QSV `src/declaration.rs:2141-2146`. |
| `CanonicalKeyUnavailable` | Collection ordering cannot obtain the canonical key of a member (a pair `compare_keys` has no key for; two composites of different declarations have one, [FR-377](./FR-377-composite-canonical-key-across-declarations.md)): kernel `sort_by_key` in `src/collection.rs`; RT `src/exact/collection.rs:301-310`. |
| `BoundedDivisionExpected` | The bounded division member path receives `IntegerDomain::Mathematical`: kernel `src/division.rs:121-128`. |
| `CollectionKindMismatch` | Equality reaches two collection values with different collection kinds: kernel `src/equality.rs:178-181`; RT `src/exact/equality.rs:308-311`. |
| `PopulationPair` | Kernel equality receives two population values, which the leaf cannot resolve: kernel `src/equality.rs:192-207`; [FR-089](./FR-089-kernel-refuses-a-population-pair.md). A population paired with another value kind uses `ValueKindMismatch`. |
| `ValueKindMismatch` | Equality reaches incompatible value variants other than the two-collection kind mismatch or two-population case, two composites of one declaration with different slot counts, two unions of different declarations, or two unions of one member with different payload lengths ([FR-379](./FR-379-carry-a-union-member-and-its-positional-payload-as-a-kernel-value.md)); two composites of different declarations are unequal, not this fault ([FR-376](./FR-376-composite-equality-across-declarations.md)): kernel catch-all arm of `plan_pairs` in `src/equality.rs`; RT `src/exact/equality.rs:324-338`. |
| `UnknownCheckedFunction` | RT `run_call` or `Frame::call` finds no named function after checking: `src/exact/expression.rs:802,922`. |
| `ForeignCheckedExpression` | RT `plan_evaluation` finds an expression checked against another package: `src/exact/expression.rs:829-834`. |
| `MeterBorrowConflict` | RT `Frame::call` or `Frame::meter` cannot acquire its shared mutable meter borrow: `src/exact/expression.rs:925-934,956-960`. |
| `PackageReentry` | PLANNED on the measured RT main; RT #161 adds the producer in `CheckedPackage::begin` for `CheckedPackage::call` or `CheckedPackage::evaluate` entering an already-active package from a host body before a nested charge. This is distinct from a shared meter borrow and a generated outcome adapter mismatch. |
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

The following eight causes specify CG's generated-oracle assignments.
Six rows have current typed checked-invariant constructor sites at CG main
`f935378e56cde1e7aaabe3d073669d9e58085bde`. The two result-membership and
owner-declaration construction rows remain planned under IR-709. These causes
name shared checked-oracle failures, not CG package or model types.

`GeneratedBodyPlaceholderInvoked` is retired. IR-595 replaced the pre-check
placeholder with concrete registered bodies; no live trigger remains. The
carrier SHALL omit this retired variant, with no replacement or reallocation.
CG SHALL NOT restore a placeholder solely to produce a cause. An unexpected
body resume remains a genuine `InternalFault`, never this retired cause or an
ordinary refusal. This retirement changes the generated allocation from nine
to eight causes and the complete carrier from thirty-two to thirty-one; the
other cause identities, payloads and triggers are unchanged.

| Cause | Trigger, target assignment and CG source status |
|---|---|
| `GeneratedArgumentShapeMismatch` | A generated scalar or equality function body receives the wrong number or shape of arguments: `src/oracle/function/owner_render.rs`, `scalar_item`/`equality_item`. |
| `GeneratedOperandKindUnsupported` | A generated equality body receives a left operand outside its classified Boolean/Integer kinds: `src/oracle/function/owner_render.rs`, `equality_item`. |
| `GeneratedUnexpectedOutcome` | A generated function body produces `Completed(value)` and the admitted QSV environment's `admits` returns exactly `Ok(false)` for the selected callable's checker-issued `CallableSelection::signature().result()`. Emit the typed checked-invariant payload at the generated result occurrence. `Ok(true)` retains the original completed value; `Err(EnvironmentFailure)` decided no membership and is not this cause. `Undefined`, `Refused` and `Incomplete` keep their original arms, payloads, locations and admitted charges. This result-membership producer and its test-only wrong-completed-value mutation are PLANNED under CG FR-021-AC-38/40, FR-042-AC-2 and IR-709 after IR-595. An exhaustive outcome adapter, absent continuation value or test-injected refusal does not establish this trigger. |
| `GeneratedIntervalInvalid` | A generated native integer bound cannot form a nonempty interval: `src/oracle/boolean_v1.rs`, `NATIVE_BOUND_HELPER`. |
| `GeneratedEnvironmentRejected` | Generated function preparation constructs the QSV environment from the selected checked-closure owner's actual nominal declarations and receives exactly `EnvironmentFailure::Refused(InvalidDeclaration)`. Emit this typed cause at the declaration-construction occurrence before body operations or body charges, retaining the original rejection in diagnostic evidence and any previously admitted charge prefix. Healthy production submits each authentic declaration once; its test-only emitted-source mutation submits one unchanged owner declaration twice to the real constructor. Default/empty environments, invented declarations, injected failures and `Limit`/`Allocation`/`Capacity` provide no producer credit. PLANNED under CG FR-021-AC-39, FR-042-AC-3 and IR-709 after IR-595. |
| `GeneratedTypeCheckRejected { cause }` | A generated equality oracle's declared comparison type fails shared semantic-value `check_type`; retain its kernel-owned `IllTypedCause`: `src/oracle/equality/mod.rs`, `oracle_{symbol}` type checks. |
| `GeneratedEqualityCheckRejected { cause }` | A generated equality body or oracle fails shared semantic-value `check_equality`; retain its kernel-owned `IllTypedCause`: `src/oracle/equality/mod.rs`, `oracle_{symbol}`; `src/oracle/function/owner_render.rs`, `equality_item`. The Boolean `NATIVE_EQUALITY_HELPER` uses planned kernel equality and carries its stop unchanged. |
| `GeneratedDescriptorReconstructionFailed` | A generated equality oracle receives a prepared descriptor with the wrong binding, expected package identity or context requirement, or an admitted context with the wrong package identity: `src/oracle/equality/mod.rs`, `oracle_{symbol}` prepared-environment check. Descriptor reconstruction errors while preparing that environment retain their separate typed `EnvironmentError` path. |

The two tables assign the retained causes for the target contract. The
original producer inventory was measured on quire-exact main
`497c581c9dd14eb70083488209827dd8a5b3e03a`,
quire-contract-runtime main `4180f0ea135767637b10a981393c5aef22220ace`,
and quire-semantic-value `e0ada80708fe73469923a9d6c2263cfebc0087a0`.
The `NativeIntegerCapacityBreach` row records the later kernel producer added
by IR-705 in `src/native_integer.rs`. The two CG rows without reachable
current triggers remain required target assignments. FR-042-AC-1 and TC-057
cannot be credited as complete until
their producer controls pass. The measured RT revision still has depth-stop
producers; RT FR-273 must retire them with explicit frames and `work_units`
fuel before RT pins the kernel's closed thirty-one-cause API. QSV's
`src/declaration.rs:2146` uses
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

### Generated body carrier and leaf failure boundary

The two generated-function triggers above SHALL follow the producer contract
owned by [CG FR-042](ix://agent-ix/quire-contract-codegen/FR-042) and
CG FR-021. At that body boundary the generated `quire_eval::Body::resume`
SHALL return the located typed kernel payload inside
`Ok(Transition::Done(Evaluation { outcome: FamilyOutcome::Evaluated(
Outcome::Refused(Refusal::CheckedInvariant { cause })), .. }))`.
The location SHALL name the original generated result occurrence for
`GeneratedUnexpectedOutcome`, or the declaration-construction occurrence for
`GeneratedEnvironmentRejected`. The leaf session forwards a host body's
`Done` evaluation; it does not automatically promote this carrier to an
`InternalFault`. The cause remains an internal failure with no catalog code
or catalog cause. A downstream terminal consumer SHALL retain that
classification and typed payload; the carrier's `Refused` arm SHALL NOT make
it an ordinary catalog refusal, replay inconclusive result or successful proof.

A generated checked-result mismatch SHALL NOT instead be reported as
`Err(InternalFault)` or `BodyFailure::Fault`: those channels denote a broken
body/scheduler invariant, not either specified generated result check.
When QSV `admits` returns `Err(EnvironmentFailure)`, no membership decision
exists; that failure SHALL retain its complete owner-typed payload through
`BodyFailure::QsvEnvironment` and `CallFailure::Body`. For environment
construction, only the exact `Refused(InvalidDeclaration)` trigger above
selects `GeneratedEnvironmentRejected`; every other failure SHALL keep its
owner-defined typed path. Limits, allocation and capacity failures SHALL NOT
be relabelled as either checked-invariant cause, a completed value or a body
invariant fault. Cancellation SHALL retain the signal observed by the same
QSV operation, distinct from un-cancelled work exhaustion. The leaf owns this
failure transport; Exact adds no dependency on its types and defines no copy
of that envelope. A genuine body implementation invariant uses
`BodyFailure::Fault` / `CallFailure::Body`; a leaf scheduler invariant uses
`CallFailure::Fault`. Neither channel supplies evidence for the generated
checked-result causes.

### Received-payload interpretation

For **every** `CheckedInvariant` cause, `Refusal::code()` and
`Refusal::cause()` SHALL return `None`. The typed cause SHALL remain available
to Rust callers through the variant payload and `Refusal` equality; neither
method SHALL synthesize a catalog spelling from it. QSpec FR-096 and FR-100
and QSL's FR-096/FR-100 mapping SHALL treat every such *received* payload as
an internal fault, not an ordinary refusal record. At S6a,
`Machine::stopped` SHALL return `Err(InternalFault)` with code
`runtime_invariant`, stage `S6a` and invariant `checked-program-invariant`
for either located or unlocated stops; it returns no `Evaluation` holding the
checked-invariant refusal. The QSL reference interpreter's public call path SHALL carry that fault as
`CallFailure::Fault`. The generated host-body carrier above is a separate
boundary: it retains the located typed payload in its `Done` evaluation,
without changing the internal-failure classification. A replay fault SHALL settle as `Failed`, category
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
| FR-369-AC-2 | Every kernel producer in collection, value, division and equality returns exactly its table cause; two population values return `PopulationPair`, a population paired with another kind returns `ValueKindMismatch`, and unlike collection kinds return `CollectionKindMismatch`. Two composites of different declarations return `false` and a composite paired with a non-composite returns `ValueKindMismatch` (FR-376-AC-1, FR-376-AC-6). | Test |
| FR-369-AC-5 | For every typed checked-invariant cause, `Refusal::code()` and `Refusal::cause()` return `None`, and `Eq` distinguishes otherwise equal refusals whose causes differ. Ordinary kernel refusal codes and causes are unchanged. | Test |
| FR-369-AC-6 | Every typed cause interpreted by QSL's reference `Machine::stopped` follows its existing internal-fault path: no refusal record or `Evaluation`, `runtime_invariant`/`S6a`/`checked-program-invariant` at S6a, `CallFailure::Fault` at that public interpreter call, and `Failed` with unavailable basis for an internal replay fault. Neither a received interpreter stop nor the generated-body carrier in AC-10 becomes an ordinary replay refusal, `Inconclusive`, `Incomplete(Cancelled)`, a proved result or a certification rejection; an unknown received cause retains fault provenance. | Inspection |
| FR-369-AC-8 | The public carrier contains no QSL model/checker/catalog type or dependency on RT or QSV, RT and QSV own no second refusal enum or compatibility shim, and an exhaustive source inventory finds every kernel, RT, QSV and CG production checked-invariant constructor assigned exactly one cause from the two tables. | Inspection |
| FR-369-AC-10 | Inspection against CG FR-021-AC-38..40 / FR-042-AC-2/3 finds the same two generated-function triggers, checker-issued result type and genuine owner declaration inputs. A proven wrong completed value or exact invalid-declaration rejection uses the located typed kernel payload in `Ok(Transition::Done(Evaluation))`; undecided QSV failures retain `BodyFailure::QsvEnvironment` / `CallFailure::Body`, never either checked-invariant cause or `BodyFailure::Fault`. A body or scheduler implementation fault retains its distinct fault channel. All eight CG cause assignments and the closed thirty-one-cause carrier remain required. Exact carrier construction tests supply no evidence that the planned generated producers execute; CG owns those tests. | Inspection |

## Status

The IR-1021 cause-table/rustdoc alignment specifies the generated host-body
boundary separately from the reference interpreter's received-payload mapping.
It neither implements CG's planned producer nor credits its execution or
storage-failure evidence. IR-709 owns those controls after IR-595.
The kernel carrier and kernel producer tests exist. `src/outcome.rs` exports
the pre-retirement thirty-two-cause enum without `CallDepthExceeded`.
Removing the retired placeholder variant and updating the exhaustive
`tests/checked_invariant.rs` controls remain implementation work under IR-709;
this specification-only correction does not claim that code is delivered. The RT FR-273
explicit-frame/work-fuel cutover must remove its depth-stop producers before
it pins this kernel API. IR-712 owns QSV producers, IR-709 owns CG producers
and consumers, and QSL's direct constructor migration is required before it
pins this API. This carrier change alone does not establish the downstream
producer and mapping criteria.

## Dependencies

- Kernel [FR-090](./FR-090-kernel-outcome-carries-no-caller-cause.md),
  [FR-096](./FR-096-kernel-refusal-code-and-cause.md) and
  [FR-089](./FR-089-kernel-refuses-a-population-pair.md) are amended with this
  carrier's ownership, `None` mapping and population-specific cause.
- [TC-917](../test-cases/TC-917-typed-checked-invariant-causes.md) covers the
  kernel mapping and public shape. RT FR-273/TC-194 must test explicit frames,
  `work_units` exhaustion and the absence of a depth-specific outcome; RT's
  typed-producer tests must retain unknown name, foreign expression and meter
  borrow through the shared type. RT's
  collection/equality/composite/quantity cases need matching tagged tests.
  [QSV FR-122](ix://agent-ix/quire-semantic-value/FR-122) owns the corresponding QSV internal-fault controls; [RT FR-278](ix://agent-ix/quire-contract-runtime/FR-278) and [CG FR-042](ix://agent-ix/quire-contract-codegen/FR-042) own their producer controls.
  CG FR-018/FR-021 must migrate generated constructors and replay consumers;
  QSpec/QSL FR-096/FR-100 must migrate unit matches. QSL's two production
  constructor sites additionally need a QSL-owned typed-cause/family-fault
  ruling *before* the QSL lock pins the kernel API. The QSL test-support
  fixture in `qsl-semantics/src/check/family.rs` must also stop constructing
  `CallDepthExceeded` before its kernel pin advances. These are downstream
  code/spec changes outside this kernel carrier slice.
