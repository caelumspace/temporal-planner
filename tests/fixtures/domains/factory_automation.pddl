(define (domain factory-automation)
  (:requirements :strips :typing :durative-actions :numeric-fluents)

  (:types
    machine product ingredient location worker
  )

  (:predicates
    (at ?w - worker ?l - location)
    (machine-at ?m - machine ?l - location)
    (product-at ?p - product ?l - location)
    (ingredient-at ?i - ingredient ?l - location)
    (machine-available ?m - machine)
    (worker-available ?w - worker)
    (product-ready ?p - product)
    (ingredient-processed ?i - ingredient)
  )

  (:functions
    (production-time ?m - machine ?p - product)
    (processing-time ?m - machine ?i - ingredient)
  )

  (:action move-worker
    :parameters (?w - worker ?from - location ?to - location)
    :precondition (at ?w ?from)
    :effect (and (not (at ?w ?from))
                 (at ?w ?to))
  )

  (:durative-action process-ingredient
    :parameters (?w - worker ?m - machine ?i - ingredient ?l - location)
    :duration (= ?duration (processing-time ?m ?i))
    :condition (and (at start (at ?w ?l))
                    (at start (machine-at ?m ?l))
                    (at start (ingredient-at ?i ?l))
                    (at start (machine-available ?m))
                    (at start (worker-available ?w)))
    :effect (and (at start (not (machine-available ?m)))
                 (at start (not (worker-available ?w)))
                 (at end (ingredient-processed ?i))
                 (at end (machine-available ?m))
                 (at end (worker-available ?w)))
  )

  (:durative-action produce-product
    :parameters (?w - worker ?m - machine ?p - product ?l - location)
    :duration (= ?duration (production-time ?m ?p))
    :condition (and (at start (at ?w ?l))
                    (at start (machine-at ?m ?l))
                    (at start (machine-available ?m))
                    (at start (worker-available ?w)))
    :effect (and (at start (not (machine-available ?m)))
                 (at start (not (worker-available ?w)))
                 (at end (product-ready ?p))
                 (at end (product-at ?p ?l))
                 (at end (machine-available ?m))
                 (at end (worker-available ?w)))
  )

  (:action transport-product
    :parameters (?w - worker ?p - product ?from - location ?to - location)
    :precondition (and (at ?w ?from)
                       (product-at ?p ?from)
                       (worker-available ?w))
    :effect (and (not (product-at ?p ?from))
                 (product-at ?p ?to)
                 (at ?w ?to)
                 (not (at ?w ?from)))
  )
)
