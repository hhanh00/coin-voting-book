# Ballot Verification

- Voting Server (VS) receives the vote
- VS checks the ZKP of each action
- VS checks the signatures (spend + binding)
- VS checks that the election domain nullifier was not used before
- If VS can decrypt the encrypted note, it checks that it is
valid
    - correct address
    - correct cmx
- VS stores the `nf` and the encrypted note

The encrypted note does not have the zip-212 flag or
a memo.
