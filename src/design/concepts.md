# Concepts

## Election Hash

The Election Hash EH is 32-byte hash that uniquely identifies
a given election.

EH: \\( \mathbb{F}\_{q\_\mathbb P} \\)

\\[
\mathsf{EH} = \operatorname{ToBase}^{\mathsf{Orchard}}(
\\operatorname{Blake2b-512}_{\mathsf {PersoEH}}(name))
\\]

where name is the UTF-8 byte representation of the election name,
and PersoEH is `b"ZcashVote_domain"`

## Recap on Zcash

The main state of the zcash cryptocurrency is a set of notes that record the association of an amount of ZEC to an address[^1].

A user indirectly owns ZEC by knowing the secret key matching the address of a note. The secret is needed when building transactions.

A transaction expresses the destruction of a previous note owned by the sender and the creation of a new note owned by the recipient.

Before Zcash, notes were "transparent." The address and the amount of a note are public information and are recorded in the blockchain. This lets node validators verify the transactions. They eliminate any invalid transactions and keep the blockchain clear.

However, this mechanism exposes everyone's transaction history. Zcash adds privacy by encrypting the notes, making them "shielded."

### Before Zcash

A transaction has inputs and outputs, where outputs are *newly* created
notes (address & value), and inputs are *old* notes referenced
by the transaction that created them: transaction hash & output index[^2].

### With Zcash

The shielded transactions use note commitments in place of plain notes.
The plain notes are *encrypted* and only the recipient (and optionally
the sender) can decrypt them. Since the encrypted notes are unreadable,
they cannot be part of the validation scheme.

Instead, Zero Knowledge Proofs enforce the correct calculation and the validity
of the note commitments.

```admonish tip
The Zcash transaction has essentially the same structure than Bitcoin, but with
hashes replacing the notes. 
```

Also, Zcash does not use direct references to previous notes as inputs
because it would reveal the transaction history. Instead, inputs are
hashes (called nullifiers). They are different but derived *uniquely*
from the note commitments.

In summary, note commitments and their and their nullifiers (i.e.
anti-note commitments) make but hide the transaction graph.

## Voting Notes

Voting Notes are memo-less Orchard Notes. They have an address
and a value. The value is the voting power.

Before the Election Snapshot (at end of the registration window),
voting notes are taken to be the Orchard notes.

After the Snapshot, electors and delegates can make transactions
that destroy and create Voting notes.

Obviously, these voting notes are *not* equivalent to 
the Orchard notes from the main Zcash chain.

> Voting transactions have a different structure than Zcash
transactions and cannot be replayed on the main chain.

---
[^1]: Other data such as block height, current difficulty, etc. are part
of the state but do not play an important part in voting.
[^2]: A transaction usually has several outputs. Therefore, the index
of the output is needed.
[^3]: Voting Transactions have no fees.
