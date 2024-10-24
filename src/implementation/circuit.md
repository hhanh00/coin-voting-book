# Circuit

## Public Parameters

- Anchor (cmx): `cmx_root`
- Anchor (nf): `nf_root`
- Value Commitment X, Y
- Randomized Public Key X, Y
- Election Domain nf
- Note Commitment cmx
- Election Domain Hash

## Secret Inputs

- Nullifier Range Lower Bound: `nf_start`
- `nf_start` Nullifier Merkle Path: `nf_mp` (nf_pos + nf_path)
- `cmx` Merkle Path: `cmx_mp` (pos + auth_path)
- Value Commitment Trapdoor: `rcv`
- Spent note (old)
    - rho: nullifier of the action that created the note (*not* the nullifier
    of the note)
    - psi: derived from rho and rseed and used for cmx and nf
    - rcm: also derived from rho and rseed and used for the value commitment
    - nf = nullifier of the note, derived from the full viewing key
    - v = value of the note
    - address: gd and pkd. gd is derived from the address diversifier d.
    - cmx: derived from address, v and psi
- Signature rerandomization
    - alpha
- Address integrity
    - full viewing key: ak, nk, rivk
- Output note (new)
    - rho = nullifier of spent note
    - psi = rseed.psi(rho)
    - rcm = rseed.rcm(rho)

Actions are pairs of spends & output (with dummies if needed).
The rho of the new note is the nullifier of the spent note.

```admonish warning
Before the snapshot, rho is the Zcash Nullifier.
After the snapshot, rho is the Election Domain Nullifier.
```


## Statements

### Merkle Path Validity

- `cmx_mp` is a Merkle Path from `cmx` to `cmx_root`
- `nf_mp` is a Merkle Path from `nf_start` to `nf_root`

### Value Commitment

\\[ 
    cv = \operatorname{ValueCommitment}\_\mathsf{rcv}(\mathsf{vold} - \mathsf{vnew})
\\]

### Nullifier Integrity (old note)

\\[
\mathsf{nf} = \operatorname{DeriveNullifier}_\mathsf{nk}(\rho, \psi, \mathsf{cmx})
\\]

### Domain Nullifier Integrity (old note, after Snapshot)

\\[
\mathsf{dnf} = \operatorname{DeriveDomainNullifier}_\mathsf{nk}^\mathsf{edh}(\rho, \psi, \mathsf{cmx})
\\]

### Spend Authority

\\[
    \mathsf{rk} = \mathsf{ak} + \alpha * G
\\]

### Address Integrity of Old Note

\\[
\begin{align}
\mathsf{ivk} &= \operatorname{CommitIvk}(\mathsf{ak}, \mathsf{nk}, \mathsf{rivk}) \\\\
\mathsf{pkd} &= \mathsf{ivk}.\mathsf{gd} \\\\
\end{align}
\\]

### Note Commitment Integrity

The *old* and *new* notes must have a note commitment of:

\\[
cmx = \operatorname{NoteCommitment}(
    \mathsf{gd}, \mathsf{pkd}, \mathsf{v}, \rho, \psi)
\\]

- gd comes from the diversifier d (part of the address);
- pkd is part of the address;
- v is the value of the note;
- \\( \rho, \psi \\) are derived from the random seed, rseed
of the note.

Therefore to calculate the note commitment, you must know be
able to decrypt the note.

### Range Check on nf

- `nf` in [`nf_start`, `nf_end`]
- `nf_end` = `nf_path[0]`

### Other Orchard Constraints

net_cv = v_old - v_new
cmx_root = advice
nf_root = advice
nf_pos is even

## Notes

```admonish note
The Voting Circuit is very similar to the Orchard Circuit.
The definition of DeriveNullifier, NoteCommitment, etc.
are the same.
```

DeriveDomainNullifier is new though.

\\[
\begin{align}
\operatorname{DeriveNullifier} &= (
    \operatorname{PH}(\mathsf{nk}, \rho) + \psi).\mathsf{Nk} + \mathsf{cm} \\\\
\operatorname{DeriveDomainNullifier} &= (
    \operatorname{PH}(\mathsf{nk}, \operatorname{PH}(\mathsf{edh}, \rho)
    ) + \psi).\mathsf{Nk} + \mathsf{cm} \\\\
\end{align}
\\]

where PH is the Posseidon Hash.

> The parameters of PH used in the Orchard circuit is 
proven secure for hashing two elements of Fp, not three.

