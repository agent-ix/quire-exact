// SPDX-License-Identifier: AGPL-3.0-or-later
//! A caller's say over the heap an exact-integer operation requests (FR-396).

/// A caller's decision on heap growth, asked before each request.
///
/// A reserved integer operation calls [`Reservation::reserve`] once for every
/// limb block it is about to request from the allocator, with the full size of
/// that block in bytes (the new capacity times the limb size), and requests the
/// block only if the call returns `Ok`. The bytes asked are therefore the bytes
/// requested from the allocator, in the order the requests happen. Nothing is
/// credited back when a block is released, so the sum of the bytes asked is an
/// upper bound on the operation's peak heap.
///
/// The shape is identical to `quire-canonical`'s `Reservation`, so one caller
/// meter implements both without this leaf depending on that crate.
pub trait Reservation {
    /// Admit or refuse `bytes` of heap.
    ///
    /// # Errors
    ///
    /// [`ReservationDenied`] to refuse. The operation allocates nothing for the
    /// refused request, releases what it holds and returns the denial.
    fn reserve(&mut self, bytes: usize) -> Result<(), ReservationDenied>;
}

/// A [`Reservation`] refused a request.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ReservationDenied;

impl<R: Reservation + ?Sized> Reservation for &mut R {
    fn reserve(&mut self, bytes: usize) -> Result<(), ReservationDenied> {
        (**self).reserve(bytes)
    }
}

/// The reservation that admits every request: the unmetered operations are the
/// reserved ones run with it.
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct Unlimited;

impl Reservation for Unlimited {
    fn reserve(&mut self, _bytes: usize) -> Result<(), ReservationDenied> {
        Ok(())
    }
}
