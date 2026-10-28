; SPDX-License-Identifier: AGPL-3.0-or-later
; Negated preservation of exact signed mathematical integer product, order,
; minimum and maximum under the standard SMT-LIB integer-to-real embedding.
; No finite integer bounds or assumed arithmetic identities are supplied.
(set-logic ALL)
(declare-const i Int)
(declare-const j Int)
(assert (or
  (distinct (to_real (* i j)) (* (to_real i) (to_real j)))
  (xor (<= i j) (<= (to_real i) (to_real j)))
  (distinct (to_real (ite (<= i j) i j))
            (ite (<= (to_real i) (to_real j)) (to_real i) (to_real j)))
  (distinct (to_real (ite (<= i j) j i))
            (ite (<= (to_real i) (to_real j)) (to_real j) (to_real i)))))
(check-sat)
