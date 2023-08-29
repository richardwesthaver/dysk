;;; dysk.lisp
;; (dysk:hello) ;; => "hello from rust"
(defpackage :dysk
  (:use :cl :sb-alien)
  (:export :hello :plus :plus1))
(in-package :dysk)
(load-shared-object #P"target/release/libdysk.so")
(define-alien-routine hello c-string)
(define-alien-routine plus int (a int) (b int))
(define-alien-routine plus1 int (n int))
