# Consensus Rules

In layman terms, a cryptocurrency system must enforce the following rules
in some way.

1. Output Note must go to the recipient
1. Input Notes must exist
1. Input Notes must be yours
1. Input Notes must not be spent
1. Total value of the inputs must be equal to the total value of the outputs & fee
(Conservation of Value)

## Output Note must go to the recipient

In Bitcoin, the transaction output have the address of the recipient[^1].

> In Zcash, outputs are note commitments, i.e. hashes. However,
the note commitment is checked by the statement
[note-commitment-integrity](circuit.md#note-commitment-integrity) 
of the ZKP.

## Input Notes must exist

In Bitcoin, the transaction inputs are references to the output note
by transaction id and output position. This can easily be checked
by keeping a database of all the transactions.

In Zcash, the existence is proven by the statement
[merkle-path-validity](circuit.md#merkle-path-validity)

```admonish note
Merkle Tree are commonly used to prove that an element
is part of a public list.
```

In this case, the public list is the list of *all* the note
commitments, spent or not (there is no way of telling them apart).
Usually, the prover would present the element and a 
[Merkle Path](https://en.wikipedia.org/wiki/Merkle_tree).
The verifier checks the proof by calculating the root hash
and matching it against the public root value.

With Zcash, this verification is made in a ZKP which
*hides* the element and the Merkle Path.

> In layman terms, the ZKP states that the note
exists because the creator of the transaction
can provide the note commitment and a Merkle Path
that leads to the public root. In a sense, the ZKP
is a *proof of a proof*.

## Input Notes must be yours

Both Bitcoin and Zcash use the same technique.
The address is derived from a secret key. Knowing that secret
key proves that the sender owns the address.

> The sender proves they have the secret key by
signing the note[^2].

Technically speaking, proving ownership of the address
of a note is not the same as proving ownership of the note.

It just proves ownership *at some point in time*. It
is possible that the note is already spent.

## Input Notes must not be spent

This prevents *double spending*, i.e issuing two or more
payments that use the same note (with a correct signature).

In Bitcoin, it is avoided by simply checking that the output
is at most used once. There cannot be two transactions that
use inputs that refer to the same previous output.

In Zcash, the inputs do not refer directly to a previous output,
therefore we cannot check double spending with a database lookup.

Instead, Zcash uses *nullifiers*. A nullifier is another hash
value associated with a note. The first hash is the note commitment.

Every note has a note commitment and a nullifier, but obviously
there are not the same.

The creator of a note, the person who created the payment transaction,
can compute the note commitment. But the nullifier can only
be computed by the recipient, i.e the owner.

Zcash transactions expose the nullifiers `nf` and the note commitments
`cmx`.

Voting transactions do the same thing but with *different nullifiers*.
If the votes shared the same nullifier, then votes could be linked
between different elections.

Consider the case of a voter who keeps a stash of ZEC and votes on
two different elections with the same notes.

The Voting system eliminates this issue by defining a new nullifier
derivation rule that includes the Election Hash.

The same note has distinct Election nullifiers and cannot be linked
with the nullifier from any other election or the Zcash main chain.

However, the Election nullifier cannot be used to prevent voting
with spent note. 

### Before Snapshot (in Zcash chain)

> *Zcash Nullifiers* must not be used more than once

### After Snapshot (in Vote chain)

> *Election Domain Nullifiers* must not be used more than once.
The statements
[merkle-path-validity](circuit.md#merkle-path-validity),
[domain-nullifier-integrity](circuit.md#domain-nullifier-integrity),
and [range-check-on-nf](circuit.md#range-check-on-nf)
of the ZKP prevents reusing the *Zcash Nullifier*.

## Conservation of Value

The Conservation of Value establishes that a transaction does
not create or destroy funds/votes while keeping the values
hidden.

The idea is that when we have a point P such as
\\[ 
    P = v.G + x.H
\\]
where G and H are known points[^3],
v (value of the note)
and x (random value) are not the secret keys to P because of the other term.

But if there are summed over the whole transactions, taking v for
inputs and -v for outputs, the sum of P, let's call it Q
has only a term in H, because \\( \sum(v) = 0 \\)

\\[
    Q = \sum{v}.G + \sum{x}.H = \sum{x}.H
\\]

Then \\(\sum{x}\\) is the secret key for Q and can be used to sign
the transaction. This signature is the binding signature
as it binds the total value of the transaction.

There is one binding signature for the whole transaction.

> - The statement [value-commitment](circuit.md#value-commitment)
of the ZKP enforces the creator computed P (called value commitment cv)
correctly.
> - The binding signature ensures that overall, the value of the transaction
is zero[^4].






## Recap

The transaction is valid when the following conditions are met.

```admonish info
1. Input Note exists, i.e. it is the output of a previous transaction
1. Input Note belongs to the sender.
1. Input Note has the nullifier `nf`.
1. The `nf` has not been used before. This prevents double spending.
1. Output Note has the address of the destination
1. Output Note has the note commitment `cmx`
1. The total value of the Inputs is equal to the total value of the Outputs
(+ fee[^3])
1. `nf` and `cmx` are public
```

After the Snapshot, the consensus have additional rules to stop
double spending.

1. Input Note has the Election nullifier `domain_nf`.
1. `nf` has not been used before in the Main Zcash chain.
1. `domain_nf` and `cmx` are public (`nf` is no longer public).

The note commitments are the same before the Snapshot and diverge
because the voting transactions are different from the payment transactions.

---

[^1]: The protocol does not have the concept of addresses but works
with "scripts". However, addresses map to a given script (but the
reverse is not true).
[^2]: Or signing the transaction that spends the note.
[^3]: Generator points to be precise.
[^4]: With the fees, it is not exactly zero but the fees are known.
We can correct by adding fee.G
