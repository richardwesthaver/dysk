(require :dysk "dysk.lisp")
(time (dotimes (_ 500000000) (dysk:plus1 2)))
