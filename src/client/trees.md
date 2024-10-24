# Trees

## Blockchain Data

From `zcashd`, `zebrad` or `lightwalletd`, download
`Blocks` or `CompactBlocks` and extract the `OrchardAction`

```rust
type Hash = [u8; 32];

struct Action {
    nf: Hash,
    cmx: Hash,
}

async fn download_blocks_lwd(url: &str, start: u32, end: u32) -> Vec<Action>;
async fn download_blocks_zcashd(url: &str, start: u32, end: u32) -> Vec<Action>;
```

## Orchard Hash Function

The commitment tree hash function that combines two nodes or two leaves is the
following.

```rust
/// Orchard hash of two nodes of the CMX tree
pub fn cmx_hash(level: u8, left: &Hash, right: &Hash) -> [Hash] {
    let left = MerkleHashOrchard::from_bytes(left).unwrap();
    let right = MerkleHashOrchard::from_bytes(right).unwrap();
    let h = MerkleHashOrchard::combine(Altitude::from(level), &left, &right);
    h.to_bytes()
}
```

where level is 0 for the leaves, and increments by 1 on every layer.

If the tree has an odd number of leaves, it should be padded by adding
an empty leaf with the "empty leaf" hash value.

```rust
/// Empty Orchard CMX hash
pub fn empty_hash() -> [u8; 32] {
    MerkleHashOrchard::empty_leaf().to_bytes()
}
```

Logically speaking, the tree is a binary tree of depth 32 and starts
full of empty leaves. The `cmx_hash` combines leaves on depth 0 to
depth 1, then depth 1 to depth 2, and so on so forth, until
we reach the root.

At this point, every level of the tree has the same value obtained
by hashing two identical values from the previous layer.

As nodes are added, they replace empty leaves, progressively from
the position 0. However, empty subtrees have the same hash value
as before.

## Merkle Root

The Root hash of the tree is a public value. Transactions must
include it in their transactions.

It must match the root hash calculated at the end of a block
(from zcash or vote chain).

## Merkle Path

The Merkle Path is the combination of the following data.
- The value of the leaf (`cmx` or `nf`),
- The position of the leaf (starting at 0 for the first leaf),
- The list of 32 ommers. They are the sibling nodes on the direct path
from the leaf to the root.

The Merkle Path is an input to the ZKP circuits and should not be
sent out.

```rust
pub struct MerklePath {
    pub value: Hash,
    pub position: u32,
    pub path: [Hash; DEPTH],
}

pub fn calculate_merkle_paths(
    positions: &[u32],
    hashes: &[Hash],
) -> (Hash, Vec<MerklePath>);

```
where DEPTH = 32

`calculate_merkle_paths` takes a list of leaf positions and the list
of leaves and calculates the Merkle Path for every leaf position given.

## CMX Tree

The CMX Tree is the Merkle Tree where each leaf is the `cmx` value
from the Action in the order they appear on the Blockchain.

## NF Tree

The NF Tree is the Merkle Tree of nullifiers. They must be sorted
and a list of *complement* intervals created.
\\[ \forall i, n_i \not\in \bigcup {[a_i, b_i]}  \\]

Nullifiers are integers in the base field of the Pallas curve.

```rust
type Nullifier = Fp;

fn build_nf_ranges(nfs: impl IntoIterator<Item = Nullifier>) -> Vec<Nullifier>;
```

`build_nf_ranges` returns a list of flatten intervals as a list
\\([a_0, b_0, a_1, b_1, \dots ]\\).
