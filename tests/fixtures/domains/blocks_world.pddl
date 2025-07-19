(define (domain blocks-world)
  (:requirements :strips :typing :durative-actions)

  (:types
    block table
  )

  (:predicates
    (on ?x - block ?y - block)
    (on-table ?x - block)
    (clear ?x - block)
    (holding ?x - block)
    (arm-empty)
  )

  (:action pick-up-from-table
    :parameters (?x - block)
    :precondition (and (clear ?x)
                       (on-table ?x)
                       (arm-empty))
    :effect (and (not (on-table ?x))
                 (not (clear ?x))
                 (not (arm-empty))
                 (holding ?x))
  )

  (:action pick-up-from-block
    :parameters (?x - block ?y - block)
    :precondition (and (clear ?x)
                       (on ?x ?y)
                       (arm-empty))
    :effect (and (not (on ?x ?y))
                 (not (clear ?x))
                 (not (arm-empty))
                 (clear ?y)
                 (holding ?x))
  )

  (:action put-down-on-table
    :parameters (?x - block)
    :precondition (holding ?x)
    :effect (and (not (holding ?x))
                 (clear ?x)
                 (arm-empty)
                 (on-table ?x))
  )

  (:durative-action stack-slow
    :parameters (?x - block ?y - block)
    :duration (= ?duration 3.0)
    :condition (and (at start (holding ?x))
                    (at start (clear ?y)))
    :effect (and (at end (not (holding ?x)))
                 (at end (not (clear ?y)))
                 (at end (clear ?x))
                 (at end (arm-empty))
                 (at end (on ?x ?y)))
  )
)
