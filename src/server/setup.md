# Server Setup

## Seed

After deciding on the Election Measure and the Candidates,
the Election Officials generate a random seed phrase.

Using ZIP-32, they derive a secret key for each candidate
using the standard derivation path for accounts:
`m/32/133/<candidate>/0`

The addresses associated with the candidates are made public[^1].
They are included in the Election JSON too.

## `cmx` and `nf` roots

Once the snapshot height is reached, i.e. when voting starts,
the Voting Server (VS) should start keeping track of the `cmx`
and `nf` roots at regular intervals.
- At the snapshot height, which is also the Genesis of
the Voting Blockchain
- At frequent intervals, the VS should emit new blocks of
votes and the resulting `cmx`, `nf` hash. For example, once every
10 minutes when there is at least 1 vote.

> The VS does not need to keep all the intermediate
hash calculations of every tree for verification.
Only the root hash is required.

