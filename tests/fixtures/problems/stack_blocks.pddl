(define (problem stack-blocks)
  (:domain blocks-world)
  (:objects
    a b c - block
  )
  (:init
    (clear a)
    (clear b)
    (clear c)
    (on-table a)
    (on-table b)
    (on-table c)
    (arm-empty)
  )
  (:goal
    (and (on a b)
         (on b c))
  )
)
