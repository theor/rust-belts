 100x100 belts, 100x10 items,  move_system only:
 bench:  43,542,818 ns/iter (+/- 3,215,204)     belts -> items
 bench:  10,884,685 ns/iter (+/- 2,657,142)     items -> par belts
 bench:      42,826 ns/iter (+/- 5,720)         per belt items list
             34,940 ns/iter (+/- 7,621)

100x100 belts, 100x10 items,  move_system only:
         43,542,818 ns/iter (+/- 3,215,204)     belts -> items
         10,884,685 ns/iter (+/- 2,657,142)     items -> par belts
             42,826 ns/iter (+/- 5,720)         per belt items list
             83,712 ns/iter (+/- 12,581)        per belt items list, par mutex
             34,940 ns/iter (+/- 7,621)         per belt items list UNSAFE