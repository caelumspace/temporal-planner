(define (problem simple-delivery)
  (:domain simple-robot)
  (:objects
    robot1 - robot
    package1 package2 - package
    depot office kitchen - location
  )
  (:init
    (at robot1 depot)
    (package-at package1 depot)
    (package-at package2 depot)
    (free-hands robot1)
  )
  (:goal
    (and (delivered package1)
         (delivered package2))
  )
)
