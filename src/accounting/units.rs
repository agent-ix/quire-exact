// SPDX-License-Identifier: AGPL-3.0-or-later
//! Fixed native requests for the common charge path. Exceptional exact amounts
//! and unrestricted request collections remain behind the public Charge adapter.

use super::*;

/// Pairs of the single-pair plan of two single-occurrence values.
const PLAN_PAIRS: u64 = 1;
/// Work units `equality.plan` requires to remain: its own, the pairs and the retain.
const PLAN_RESERVATION: u64 = PLAN_PAIRS + 2;

#[derive(Clone, Copy)]
pub(crate) struct UnitsCharge {
    pub(super) point: ChargePoint,
    pub(super) first: Option<(LimitKind, u64)>,
    pub(super) second: Option<(LimitKind, u64)>,
    pub(super) work: u64,
    pub(super) results: u64,
}

impl UnitsCharge {
    pub(crate) fn new(point: ChargePoint) -> Self {
        Self {
            point,
            first: None,
            second: None,
            work: 1,
            results: 0,
        }
    }

    pub(crate) fn one(point: ChargePoint, kind: LimitKind, amount: u64) -> Self {
        Self {
            first: Some((kind, amount)),
            ..Self::new(point)
        }
    }

    pub(crate) fn two(
        point: ChargePoint,
        first: (LimitKind, u64),
        second: (LimitKind, u64),
    ) -> Self {
        Self {
            first: Some(first),
            second: Some(second),
            ..Self::new(point)
        }
    }

    /// The `equality.plan-form` request of a pair of single-occurrence values.
    pub(crate) fn equality_plan_form() -> Self {
        Self::one(
            ChargePoint::EqualityPlanForm,
            LimitKind::ValueOccurrences,
            1,
        )
        .work(2)
    }

    /// The `equality.pair` request of a one-pair plan.
    pub(crate) fn equality_pair() -> Self {
        Self::new(ChargePoint::EqualityPair)
    }

    /// The `equality.result-retain` request.
    pub(crate) fn equality_result_retain() -> Self {
        Self::new(ChargePoint::EqualityResultRetain).results(1)
    }

    pub(crate) fn work(mut self, amount: u64) -> Self {
        self.work = amount;
        self
    }

    pub(crate) fn results(mut self, amount: u64) -> Self {
        self.results = amount;
        self
    }

    pub(super) fn from_charge(charge: &Charge) -> Option<Self> {
        match &charge.storage {
            ChargeStorage::Common(common) => Some(Self {
                point: charge.point,
                first: common.sizes.first,
                second: common.sizes.second,
                work: common.work_units,
                results: common.result_units,
            }),
            ChargeStorage::General(_) => None,
        }
    }
}

impl Meter {
    pub(crate) fn charge_units(&mut self, charge: UnitsCharge) -> Result<(), Incomplete> {
        let request = self.charge_capture.as_ref().map(|capture| {
            let size_requests = charge.first.iter().count() + charge.second.iter().count();
            capture.can_record(size_requests).then(|| ChargeEvent {
                point: charge.point,
                work_units: Integer::from(charge.work),
                result_units: Integer::from(charge.results),
                work_reservation: None,
                result_reservation: None,
                sizes: charge
                    .first
                    .into_iter()
                    .chain(charge.second)
                    .map(|(kind, amount)| (kind, Integer::from(amount)))
                    .collect(),
                admission: ChargeAdmission::Denied,
            })
        });
        let result = self.charge_units_inner(charge);
        self.record_capture(request, result.is_ok());
        result
    }

    fn charge_units_inner(&mut self, charge: UnitsCharge) -> Result<(), Incomplete> {
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
                } else if first_fail.is_none_or(|(first_kind, _)| kind.index() < first_kind.index())
                {
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
            self.consumed(kind)
                .checked_add(addition)
                .filter(|total| *total <= kind.limit(&self.counters.limits))
                .ok_or_else(|| self.incomplete(kind, Integer::from(addition), point))
        };
        let work = cumulative(LimitKind::WorkUnits, charge.work)?;
        let results = cumulative(LimitKind::ResultUnits, charge.results)?;
        self.commit_charge(
            point,
            sizes,
            work,
            results,
            #[cfg(feature = "test-support")]
            record,
        );
        Ok(())
    }

    /// The `equality.plan` charge of a plan with exactly one pair, in `u64`
    /// arithmetic: the same denial, reservation, size and consumption as
    /// [`Meter::charge_plan`] with `pairs = 1`, which needs 3 remaining work
    /// units and 1 remaining result unit and consumes one work unit.
    pub(crate) fn charge_plan_one_pair(&mut self) -> Result<(), Incomplete> {
        let request = self.charge_capture.as_ref().map(|capture| {
            capture.can_record(1).then(|| ChargeEvent {
                point: ChargePoint::EqualityPlan,
                work_units: Integer::one(),
                result_units: Integer::from(0_u64),
                work_reservation: Some(Integer::from(PLAN_RESERVATION)),
                result_reservation: Some(Integer::one()),
                sizes: alloc::vec![(LimitKind::ValueOccurrences, Integer::from(PLAN_PAIRS))],
                admission: ChargeAdmission::Denied,
            })
        });
        let result = self.charge_plan_one_pair_inner();
        self.record_capture(request, result.is_ok());
        result
    }

    fn charge_plan_one_pair_inner(&mut self) -> Result<(), Incomplete> {
        let point = ChargePoint::EqualityPlan;
        let kind = LimitKind::ValueOccurrences;
        if let Some(consumed) = self.injected_denial(point) {
            return Err(Incomplete {
                limit_kind: LimitKind::WorkUnits,
                limit: consumed,
                consumed,
                next_charge: Integer::from(PLAN_RESERVATION),
                charge_point: point,
            });
        }
        if PLAN_PAIRS > kind.limit(&self.counters.limits) {
            return Err(self.incomplete(kind, Integer::from(PLAN_PAIRS), point));
        }
        let remaining = |kind: LimitKind| {
            kind.limit(&self.counters.limits)
                .saturating_sub(self.consumed(kind))
        };
        if PLAN_RESERVATION > remaining(LimitKind::WorkUnits) {
            return Err(self.incomplete(
                LimitKind::WorkUnits,
                Integer::from(PLAN_RESERVATION),
                point,
            ));
        }
        if remaining(LimitKind::ResultUnits) < 1 {
            return Err(self.incomplete(LimitKind::ResultUnits, Integer::one(), point));
        }
        let mut sizes = [None; LimitKind::COUNT];
        sizes[kind.index()] = Some(PLAN_PAIRS);
        self.capture_charge(
            &sizes,
            self.consumed(LimitKind::WorkUnits) + 1,
            self.consumed(LimitKind::ResultUnits),
        );
        let slot = &mut self.counters.consumed[kind.index()];
        *slot = (*slot).max(PLAN_PAIRS);
        let work = &mut self.counters.consumed[LimitKind::WorkUnits.index()];
        *work = work.saturating_add(1);
        self.admit(
            point,
            #[cfg(feature = "test-support")]
            AdmittedCharge {
                value_occurrences: Some(PLAN_PAIRS),
                ..AdmittedCharge::new(point)
            },
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use core::num::NonZeroU64;

    use ix_trace_rs::trace;

    use super::*;

    fn limits(work: u64, results: u64, occurrences: u64) -> ScalarLimits {
        ScalarLimits {
            integer_bits: u64::MAX,
            decimal_digits: u64::MAX,
            scale_expansion: u64::MAX,
            text_input_bytes: u64::MAX,
            text_scalars: u64::MAX,
            normalized_scalars: u64::MAX,
            unit_edges: u64::MAX,
            value_occurrences: occurrences,
            work_units: work,
            result_units: results,
        }
    }

    type Observed = (Result<(), Incomplete>, Vec<u64>, u64, Vec<ChargeEvent>);

    fn observe(mut meter: Meter, f: impl Fn(&mut Meter) -> Result<(), Incomplete>) -> Observed {
        meter.enable_charge_capture(8, 16);
        let result = f(&mut meter);
        let counters = LimitKind::ALL.iter().map(|k| meter.consumed(*k)).collect();
        let events = match meter.disable_charge_capture() {
            Some(Ok(capture)) => capture.events().to_vec(),
            _ => panic!("capture missing or truncated"),
        };
        (result, counters, meter.admission_count(), events)
    }

    #[trace("TC-944", "FR-397-AC-6")]
    #[test]
    fn the_equality_schedule_requests_are_native_common_charges() {
        // Each request the equality schedule makes (the constructors it calls) equals a `Common` (all-`u64`)
        // `Charge`, never the `General` arbitrary-precision storage.
        let plan_form = Charge::new(ChargePoint::EqualityPlanForm)
            .exact_size(LimitKind::ValueOccurrences, Integer::one())
            .work(Integer::from(2_u64));
        let pair = Charge::new(ChargePoint::EqualityPair);
        let retain = Charge::new(ChargePoint::EqualityResultRetain).results(1);
        for (charge, native) in [
            (plan_form, UnitsCharge::equality_plan_form()),
            (pair, UnitsCharge::equality_pair()),
            (retain, UnitsCharge::equality_result_retain()),
        ] {
            let common = UnitsCharge::from_charge(&charge).expect("common storage");
            assert_eq!(common.point, native.point);
            assert_eq!(common.first, native.first);
            assert_eq!(common.second, native.second);
            assert_eq!(common.work, native.work);
            assert_eq!(common.results, native.results);
        }
    }

    #[trace("TC-944", "FR-397-AC-7")]
    #[test]
    fn one_pair_plan_in_u64_equals_the_integer_plan_charge() {
        for work in 0..=5 {
            for results in 0..=2 {
                for occurrences in 0..=2 {
                    let l = limits(work, results, occurrences);
                    assert_eq!(
                        observe(Meter::new(l), Meter::charge_plan_one_pair),
                        observe(Meter::new(l), |m| m.charge_plan(&Integer::one())),
                        "{work} {results} {occurrences}"
                    );
                }
            }
        }
        let denial = InjectedDenial {
            point: ChargePoint::EqualityPlan,
            occurrence: NonZeroU64::MIN,
        };
        let build = || Meter::new(limits(9, 9, 9)).with_injected_denial(denial);
        assert_eq!(
            observe(build(), Meter::charge_plan_one_pair),
            observe(build(), |m| m.charge_plan(&Integer::one()))
        );
    }
}
