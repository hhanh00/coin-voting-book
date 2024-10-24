## Election Window

```admonish info
The election data forks from the main blockchain at the snapshot height.
Notes from the registration window make the baseline of the voting power, but then voters can delegate their votes to others.
**Delegation** works by submitting a "vote" to another user.

**Grants** are equivalent to premining.
```

Logically speaking, voters acquired voting power during the registration window. In the election window, they use their voting power by sending it to another user or voting for a candidate or a motion.

The system leverages Orchard receivers. The candidates have "addresses" in the JSON file. 

## Direct Election
Votes sent to these addresses are final. At the end, the Election Authority
tallies the votes and the winner is decided.

## Indirect Election
However, an election scheme could be "indirect" or in stages, where these candidates proceed to vote further using the votes they received in a previous stage.

```mermaid
flowchart LR
    ZHolders --> Candidates
    Candidates --> Winner
```

