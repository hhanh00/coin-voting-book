# Action

## Election Domain Hash

Compute the `edh` from the Election name

## Inputs

```rust
struct VoteInput {
    sk: Option<SpendingKey>,
    fvk: FullViewingKey,
    pos: u32,
    note: Note,
    nf_start: Nullifier,
    nf_pos: u32,
    nf_path: MerklePath,
    cmx_path: MerklePath,
}
```

- `sk` is needed if the election requires "Spend Signatures",
- `pos` is the position of the note in the `cmx` tree[^1],
- `nf_start` is the \\(a_i\\) of the interval \\([a_i, b_i]\\) where
`nf` belongs,
- `nf_pos` is the position of `nf_start` in the NF tree,
- `nf_path` and `cmx_path` are the Merkle Path obtained in the 
[previous step](trees.md#nf-tree).

Calculate the domain nullifier `dnf`.

```rust
let dnf = spend.note.nullifier_domain(&fvk, &edh);
```

## Output

The output is a voting note.

```rust
struct VoteOutput(Address, u64);
```

## Encrypted Note

The encrypted note corresponds to the CompactAction of Orchard.
For reference, it is composed of the following fields.
- zip-212 version byte: 1 byte
- address diversifier: 11 bytes
- address pkd: 32 bytes
- value: 8 bytes
For a total of 52 bytes.

It can be built using `OrchardNoteEncryption` from LRZ.

In pseudo-code
1. Set `rho` as the nullifier (or domain diversifier on the Vote Chain) 
of the spent note
1. Pick `rseed` with the OsRng
1. Create an Orchard Note 
    `Note::from_parts(recipient, NoteValue::from_raw(value), rho, rseed)`
1. Encrypt the node

```rust
let rseed = RandomSeed::random(&mut rng, &rho);
let note = Note::from_parts(recipient, NoteValue::from_raw(value), rho, rseed);
let encryptor = OrchardNoteEncryption::new(None, output.clone(), candidate, [0u8; 512]);
```

Get the ephemeral public key `epk, the encrypted note `enc` and the note
commitment `cmx`.

```rust
let epk = encryptor.epk();
let enc = encryptor.encrypt_note_plaintext()[0..52];
let cmx = note.commitment();
```

1. Calculate the vote net value as input value - output value.
1. Pick a random trapdoor value `rcv`
1. Accumulate `rcv` over the transaction
1. Derive the net value commitment

```rust
let value_net = spend.note.value() - output.value();
let rcv = ValueCommitTrapdoor::random(&mut rng);
total_rcv = total_rcv + &rcv;
let cv_net = ValueCommitment::derive(value_net, rcv.clone());
```

## Randomized public key

1. Pick a random scalar \\( \alpha \\) in Fq (scalar field of Pallas)
1. Derive the spend authorizing key from the spending key
1. Add \\( \alpha \\) and derive the *public key*

```rust
let alpha = Fq::random(&mut rng);
let spk = SpendAuthorizingKey::from(&spend.sk);
let rk = spk.randomize(&alpha);
```

## Zero Knowledge Proof

Collect the public data (instance data):
- `cmx_root`, `nf_root`
- `cv_net`
- `dnf`
- `rk`
- `cmx`
- `edh`

And the secret data (advice data)
- `dnf`
- `nf_start`
- `nf_path`
- `fvk`
- `spend_note`
- `cmx_path`
- `output_note`
- `alpha`
- `rcv`

Then call the ZKP builder for the voting circuit as described
in [Circuit]({{rootUrl}}/implementation/circuit.html).

## Ballot Action

Collect
- `cv`
- `rk`
- `dnf`
- `cmx`
- `epk`
- `enc`

and form the BallotAction

```rust
struct BallotAction {
    cv: Hash,
    rk: Hash,
    nf: Hash,
    cmx: Hash,
    epk: Hash,
    enc: [u8; 52],
}
```

## Ballot Data

Collect all the BallotActions into a BallotData

```rust
struct BallotData {
    actions: Vec<BallotAction>,
}
```

## Sighash

Compute the sighash of the BallotData as

```rust
fn sig_hash(&self) -> Hash {
    let bin_data = serde_cbor::to_vec(&self).unwrap();
    let p = Params::new()
        .hash_length(32)
        .personal(b"Ballot______Data")
        .hash(&bin_data);
    p.as_bytes().try_into().unwrap()
}
```

It is the Blake2b-256 hash of the CBOR serialization of 
BallotData with Personalization `Ballot______Data`.

## Binding Signature

Use the sighash and `total_rcv` to compute the Binding Signature

```rust
let bsk: SigningKey<Binding> = rcv.to_bytes();
let binding_signature = bsk.sign(&mut rng, sig_hash);
```

## Spend Signature

Sign the sighash using the public key `rk`, `sighash`
and SpendAuth.

## Ballot Envelope

Combine the Ballot Data, Binding Signature, Input Signatures
and proofs into the Ballot Envelope.

```rust
type Signature = [u8; 64];

struct BallotEnvelope {
    data: BallotData,
    input_signature: Vec<Signature>,
    binding_signature: Signature,
    proofs: Vec<Vec<u8>>,
}
```

## Send to Server

Serialize BallotEnvelope as CBOR and send the bytes to the
Voting Server.

---
[^1]: Relative to the registration window, not to the activation
of Orchard.
[^2]: Or domain diversifier af