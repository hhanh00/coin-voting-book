# Name
The *name* of the election is displayed on the voting UI and must uniquely
identify the election. It gets hashed and serves as a way to distinguish between
ballots.

```admonish info
Ballots that use the same note cannot be linked
because of the unique election ID
```

It is important to make sure that another election does not use
the same name.

```admonish tip
Pick a name that represents the election with no ambiguity. Remember that any change in the name will invalidate all the ballots and force voters to resubmit new ones.
```

## Examples

- Good choices: "Devfund 2024 - ZCG - Jul 2024"
- Bad choices: 
    - "Devfund 2024", no mention of the Election Authority
    - "Devfund 2024 - ZCG" - there was going to be more than one vote
- Questionable choice: 
    - "Devfund 2024 - ZCG - 2671000" - implies that the snapshot height is set (could be intentional)

