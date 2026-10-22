// SPDX-License-Identifier: AGPL-3.0-or-later
//! Independent field-ordered mathematical transition for native unit requests.

use super::{ChargePoint, LimitKind};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct Denial {
    pub kind: LimitKind,
    pub limit: u64,
    pub consumed: u64,
    pub next: u64,
    pub point: ChargePoint,
}

pub(super) const KINDS: [LimitKind; 10] = [
    LimitKind::IntegerBits,
    LimitKind::DecimalDigits,
    LimitKind::ScaleExpansion,
    LimitKind::TextInputBytes,
    LimitKind::TextScalars,
    LimitKind::NormalizedScalars,
    LimitKind::UnitEdges,
    LimitKind::ValueOccurrences,
    LimitKind::WorkUnits,
    LimitKind::ResultUnits,
];

pub(super) fn transition(
    limits: [u64; 10],
    consumed: [u64; 10],
    requests: [Option<(LimitKind, u64)>; 2],
    work: u64,
    results: u64,
    point: ChargePoint,
) -> Result<[u64; 10], Denial> {
    macro_rules! size {
        ($index:literal, $kind:ident) => {{
            let failing = |request: Option<(LimitKind, u64)>| {
                request
                    .filter(|(kind, amount)| *kind == LimitKind::$kind && *amount > limits[$index])
            };
            if let Some((kind, next)) = failing(requests[0]).or_else(|| failing(requests[1])) {
                return Err(Denial {
                    kind,
                    limit: limits[$index],
                    consumed: consumed[$index],
                    next,
                    point,
                });
            }
        }};
    }
    size!(0, IntegerBits);
    size!(1, DecimalDigits);
    size!(2, ScaleExpansion);
    size!(3, TextInputBytes);
    size!(4, TextScalars);
    size!(5, NormalizedScalars);
    size!(6, UnitEdges);
    size!(7, ValueOccurrences);
    size!(8, WorkUnits);
    size!(9, ResultUnits);
    let mut output = consumed;
    macro_rules! cumulative {
        ($index:literal, $kind:ident, $amount:expr) => {
            output[$index] = consumed[$index]
                .checked_add($amount)
                .filter(|total| *total <= limits[$index])
                .ok_or(Denial {
                    kind: LimitKind::$kind,
                    limit: limits[$index],
                    consumed: consumed[$index],
                    next: $amount,
                    point,
                })?;
        };
    }
    cumulative!(8, WorkUnits, work);
    cumulative!(9, ResultUnits, results);
    macro_rules! high_water {
        ($index:literal, $kind:ident) => {{
            let amount = |request: Option<(LimitKind, u64)>| {
                request
                    .filter(|(kind, _)| *kind == LimitKind::$kind)
                    .map_or(0, |(_, amount)| amount)
            };
            output[$index] = consumed[$index]
                .max(amount(requests[0]))
                .max(amount(requests[1]));
        }};
    }
    high_water!(0, IntegerBits);
    high_water!(1, DecimalDigits);
    high_water!(2, ScaleExpansion);
    high_water!(3, TextInputBytes);
    high_water!(4, TextScalars);
    high_water!(5, NormalizedScalars);
    high_water!(6, UnitEdges);
    high_water!(7, ValueOccurrences);
    Ok(output)
}
