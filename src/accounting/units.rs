// SPDX-License-Identifier: AGPL-3.0-or-later
//! Fixed native requests for the common charge path. Exceptional exact amounts
//! and unrestricted request collections remain behind the public Charge adapter.

use super::*;

#[derive(Clone, Copy)]
pub(crate) struct UnitsCharge {
    point: ChargePoint,
    first: Option<(LimitKind, u64)>,
    second: Option<(LimitKind, u64)>,
    work: u64,
    results: u64,
}

impl UnitsCharge {
    pub(crate) fn new(point: ChargePoint) -> Self {
        Self { point, first: None, second: None, work: 1, results: 0 }
    }

    pub(crate) fn one(point: ChargePoint, kind: LimitKind, amount: u64) -> Self {
        Self { first: Some((kind, amount)), ..Self::new(point) }
    }

    pub(crate) fn two(point: ChargePoint, first: (LimitKind, u64), second: (LimitKind, u64)) -> Self {
        Self { first: Some(first), second: Some(second), ..Self::new(point) }
    }

    pub(crate) fn results(mut self, amount: u64) -> Self {
        self.results = amount;
        self
    }

    pub(super) fn from_charge(charge: &Charge) -> Option<Self> {
        if !charge.sizes.overflow.is_empty() { return None; }
        let request = |slot: &Option<(LimitKind, ChargeAmount)>| match slot {
            Some((kind, amount)) => Some(Some((*kind, amount.units()?))),
            None => Some(None),
        };
        Some(Self {
            point: charge.point,
            first: request(&charge.sizes.first)?,
            second: request(&charge.sizes.second)?,
            work: charge.work_units.units()?,
            results: charge.result_units.units()?,
        })
    }
}

impl Meter {
    pub(crate) fn charge_units(&mut self, charge: UnitsCharge) -> Result<(), Incomplete> {
        let point = charge.point;
        if let Some(consumed) = self.injected_denial(point) {
            return Err(Incomplete {
                limit_kind: LimitKind::WorkUnits,
                limit: consumed,
                consumed,
                next_charge: Integer::from(charge.work),
                charge_point: point,
            });
        }
        let mut sizes = [None; LimitKind::COUNT];
        let mut first_fail: Option<(LimitKind, u64)> = None;
        #[cfg(feature = "test-support")]
        let mut record = AdmittedCharge::new(point);
        let mut observe = |request: Option<(LimitKind, u64)>| {
            if let Some((kind, amount)) = request {
                if amount <= kind.limit(&self.counters.limits) {
                    let slot = &mut sizes[kind.index()];
                    *slot = Some(slot.map_or(amount, |previous: u64| previous.max(amount)));
                    #[cfg(feature = "test-support")]
                    record.observe(kind, amount);
                } else if first_fail.is_none_or(|(first_kind, _)| kind.index() < first_kind.index()) {
                    first_fail = Some((kind, amount));
                }
            }
        };
        observe(charge.first);
        observe(charge.second);
        if let Some((kind, amount)) = first_fail {
            return Err(self.incomplete(kind, Integer::from(amount), point));
        }
        let cumulative = |kind: LimitKind, addition: u64| {
            self.consumed(kind).checked_add(addition)
                .filter(|total| *total <= kind.limit(&self.counters.limits))
                .ok_or_else(|| self.incomplete(kind, Integer::from(addition), point))
        };
        let work = cumulative(LimitKind::WorkUnits, charge.work)?;
        let results = cumulative(LimitKind::ResultUnits, charge.results)?;
        self.commit_charge(point, sizes, work, results,
            #[cfg(feature = "test-support")]
            record,
        );
        Ok(())
    }
}
