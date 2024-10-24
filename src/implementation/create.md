# Vote to Ballot

## Prepare Election

The Election Authority:
- Decides the Registration Window (and the note filtering if used)
- States the Purpose (or Question) and Choices

The server:
- Generates a random seed phrase
- Derives an Orchard address per answer
- Builds the initial cmx & nf trees
- Monitors the Blockchain and update trees until the snapshot
is reached. The setup is described in 
[Server Setup]({{rootUrl}}/server/setup.html)

Optional:
- The Election Authority sets up a website for the election
with information about the purpose, the candidates, etc.
- The URL of the voting server is published online.
Alternatively, the URL is sent by email, etc.

## Elector Registration

- Elector moves coins into Orchard
or refreshes existing Orchard notes if necessary.

> A wide window of registration reduces the need
to move Orchard Notes but increases the reference data
set that electors need to download and process.

## Vote Preparation

- Voting app VA (or wallet with voting capabilities)
downloads election definition from server
- VA downloads cmx and nf tree from either
    - server
    - lightwalletd, and process the raw block data
    to reconstruct the election data[^1]
    - get a bootstrap file from torrent or from ipfs
- VA caches the trees
- Once the snapshot height is reached, VA can
compute the tree hashes and cache the result

## Vote or Delegation

- User selects a candidate or candidates
and a quantity of ZEC
- VA builds a voting transaction with 1 or many
recipients
- VA adds a change output if needed
- VA computes ZKP for each action
- VA computes binding signature
- VA signs each input
- VA transmits transaction to Voting Server

## Counting & Audit

## Important Note about nf Tree

The `nf` tree is a non-inclusion tree. It is not
an incremental merkle tree like the `cmx` and
cannot be optimized because nullifier nodes
are not only appended.

The client has to download and *keep* the complete set
of nullifiers because the tree is sorted and new
nullifiers can end up anywhere.

Every action has a nullifier and a note commitment.
Some of them are dummies but there is no way to tell
which ones.

Both nullifier `nf` and note commitment `cmx` take 32 bytes
each. An action takes 64 bytes to download.
The `cmx` does not need to be kept, but the `nf` does.
Therefore it takes about 32 bytes of storage.
It takes about N hash calculations to compute
the Merkle Path when N is the number of leaves.
We can compute the paths for multiple notes
simultaneously. But when new leaves are added,
the intermediate hashes must be recalculated[^2].
The `nf` tree has 2N leaves and the `cmx` tree has N
leaves.

| Number of Actions | N   |
|-------------------|-----|
| Bytes downloaded  | 64N |
| Bytes stored      | 32N |
| Hash Calculations | 3N  |
| NF Tree Cache (bytes) | 64N |


```admonish warning
The Registration Window has a big impact on N.

At this time, the number of Orchard actions is ~60 million.
Without spam filtering, it takes ~10 Gb to download.
With spam filtering, it is about ~1.5 Gb.
```

---
[^1]: cmx tree is not the same as the Zcash commitment
tree because of the filtering and the block range.
[^2]: If we know the new leaves are appended, there are
optimizations that allow to reuse the previous calculations.
They are not likely usable because the parameters
are different between elections.
