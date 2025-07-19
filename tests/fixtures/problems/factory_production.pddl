(define (problem factory-production)
  (:domain factory-automation)
  (:objects
    worker1 worker2 - worker
    machine1 machine2 - machine
    product1 product2 - product
    ingredient1 ingredient2 ingredient3 - ingredient
    station1 station2 warehouse - location
  )
  (:init
    (at worker1 station1)
    (at worker2 station2)
    (machine-at machine1 station1)
    (machine-at machine2 station2)
    (ingredient-at ingredient1 station1)
    (ingredient-at ingredient2 station2)
    (ingredient-at ingredient3 warehouse)
    (machine-available machine1)
    (machine-available machine2)
    (worker-available worker1)
    (worker-available worker2)
    (= (production-time machine1 product1) 4.0)
    (= (production-time machine2 product2) 6.0)
    (= (processing-time machine1 ingredient1) 2.0)
    (= (processing-time machine2 ingredient2) 3.0)
  )
  (:goal
    (and (product-ready product1)
         (product-ready product2)
         (ingredient-processed ingredient1)
         (ingredient-processed ingredient2))
  )
)
