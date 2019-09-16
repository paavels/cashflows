# cashflows.rs

Command line utilities for producing and manipulating cashflows. Idea is to have low level command line utilities that perform individual tasks and can be combined for more complex scenarios. 

## create_cashflow

Produces cashflow by combining development patterns (derived from cumulative development factors (CDF)) and amounts file.
See https://en.wikipedia.org/wiki/Chain-ladder_method

Usage:

```
    create_cashflow.exe pattern.txt amounts.txt cashflow.txt
```

### TODO:
 - [x] delimiter setting from command line arguments
 - [ ] command line argument to switch from cumulative values to incremental values
 - [ ] think about unix-pipe style approach
 - [ ] proper error handling
 - [ ] pattern - sort by DevelopmentPeriod
