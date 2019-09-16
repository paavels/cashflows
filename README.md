# Cashflows

Command line utilities for producing and manipulating cashflows. Idea is to have low level command line utilities that perform individual tasks and can be combined for more complex scenarios. 

## create_cashflow

Produces cashflow by combining payment pattern and amounts file.

Usage:

```
    create_cashflow.exe pattern.txt amounts.txt cashflow.txt
```

### TODO:
 - [ ] delimiter setting from command line arguments
 - [ ] command line argument to switch from cumulative values to incremental values
 - [ ] think about unix-pipe style approach

