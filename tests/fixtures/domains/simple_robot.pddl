(define (domain simple-robot)
  (:requirements :strips :typing :durative-actions)

  (:types
    robot location package
  )

  (:predicates
    (at ?r - robot ?l - location)
    (package-at ?p - package ?l - location)
    (holding ?r - robot ?p - package)
    (delivered ?p - package)
    (free-hands ?r - robot)
  )

  (:action move
    :parameters (?r - robot ?from - location ?to - location)
    :precondition (at ?r ?from)
    :effect (and (not (at ?r ?from))
                 (at ?r ?to))
  )

  (:action pick-up
    :parameters (?r - robot ?p - package ?l - location)
    :precondition (and (at ?r ?l)
                       (package-at ?p ?l)
                       (free-hands ?r))
    :effect (and (not (package-at ?p ?l))
                 (not (free-hands ?r))
                 (holding ?r ?p))
  )

  (:action drop
    :parameters (?r - robot ?p - package ?l - location)
    :precondition (and (at ?r ?l)
                       (holding ?r ?p))
    :effect (and (not (holding ?r ?p))
                 (package-at ?p ?l)
                 (free-hands ?r))
  )

  (:durative-action deliver
    :parameters (?r - robot ?p - package ?dest - location)
    :duration (= ?duration 2.0)
    :condition (and (at start (holding ?r ?p))
                    (at start (at ?r ?dest)))
    :effect (and (at end (delivered ?p))
                 (at end (not (holding ?r ?p)))
                 (at end (free-hands ?r)))
  )
)
