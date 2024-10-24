# Ballot

The Ballot is the data sent from the Client to the Voting Server
that represents a vote for a candidate/question or a delegation
to another user(s).

The voter partially use their voting power by adding a
"change" vote output similarly to standard payment transactions.

## BallotEnvelope

The BallotEnvelope is the binary representation of the ballot.

It has the following data.

- Ballot Payload (see next paragraph),
- Binding Signature
- ZK proofs of the Ballot Actions

## BallotPayload

The Ballot Data is the non-malleable part of the Ballot.
The SigHash is calculated on the Ballot Data.

It is a vector of Ballot Actions.

## BallotAction

The Ballot Action represents an individual transfer
of voting power. It burns a previous output
and mints a new one. If there are more spends than
outputs, the action includes dummy data. Dummy notes
have a value equal to 0.

The Ballot Action has the following fields.

- `cv`: Value commitment,
- `rk`: Public key of the rerandomized authorization key,
- `nf`: Input Note Election domain nullifier,
- `cmx`: Output Note commitment,
- `epk`: Ephemeral Public Key
- `enc`: Encrypted New Note

Note that it does not have `cout` and `enc` does not have a memo.

## VoteInput

In addition to the BallotData that is public information,
the ballot app has the notion of a VoteInput and VoteOutput
in the decrypted Vote.

A Vote Input has the following fields.

- `sk`: The spending key used to sign the input,
- `fvk`: The corresponding full viewing key,
- `pos`: The position of the note in the note commitment tree,
- `note`: The note as an Orchard plain note,
    - address
    - value
    - nullifier (in the Zcash chain)
    - random seed
- `nf_start`: The lower bound of the nullifier range where
this note belongs,
- `nf_path`: The Merkle Authentication Path for `nf_start`
- `cmx_path`: The Merkle Authentication Path for the note commitment `cmx`

## VoteOutput

A vote output has the following fields:

- address
- value

(no memo field)

## Encrypted Note

The encrypted note is the encryption of (d, pkd, value)
using the same algorith as Orchard.
