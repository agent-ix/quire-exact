; SPDX-License-Identifier: AGPL-3.0-or-later
; Negation of enclosure over all mathematical reals. No native-width bound
; narrows this theorem. Native integer application also requires the Rust
; bridges and the integer-to-real embedding theorem in composition.json.
(set-logic QF_NRA)
(declare-const a Real)
(declare-const b Real)
(declare-const c Real)
(declare-const d Real)
(declare-const x Real)
(declare-const y Real)
(assert (<= a b))
(assert (<= c d))
(assert (<= a x))
(assert (<= x b))
(assert (<= c y))
(assert (<= y d))
(assert (or
  (and (< (* x y) (* a c)) (< (* x y) (* a d))
       (< (* x y) (* b c)) (< (* x y) (* b d)))
  (and (> (* x y) (* a c)) (> (* x y) (* a d))
       (> (* x y) (* b c)) (> (* x y) (* b d)))))
(check-sat)
