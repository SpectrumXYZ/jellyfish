// Copyright (c) 2022 Espresso Systems (espressosys.com)
// This file is part of the Jellyfish library.

// You should have received a copy of the MIT License
// along with the Jellyfish library. If not, see <https://mit-license.org/>.

//! Merkle Tree traits and implementations
pub mod append_only;
pub mod examples;
pub mod macros;
pub mod sparse_merkle_tree;

mod internal;

use crate::errors::PrimitivesError;
use ark_ff::Field;
use ark_std::{borrow::Borrow, fmt::Debug, ops::AddAssign, string::ToString, vec, vec::Vec};
use num_bigint::BigUint;
use num_traits::ToPrimitive;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
/// The result of querying at an index in the tree
pub enum LookupResult<F, P> {
    /// The value at the given index, and a proof of validity
    Ok(F, P),
    /// The index is valid but we do not have the leaf in memory
    NotInMemory,
    /// The index is outside the occupied range in the tree
    EmptyLeaf,
}

impl<F, P> LookupResult<F, P> {
    /// Assert the lookup result is Ok.
    pub fn expect_ok(self) -> Result<(F, P), PrimitivesError> {
        match self {
            LookupResult::Ok(x, proof) => Ok((x, proof)),
            LookupResult::NotInMemory => Err(PrimitivesError::InternalError(
                "Expected Ok, found NotInMemory".to_string(),
            )),
            LookupResult::EmptyLeaf => Err(PrimitivesError::InternalError(
                "Expected Ok, found EmptyLeaf".to_string(),
            )),
        }
    }
}

/// An element of a Merkle tree.
pub trait Element: Clone + Eq + PartialEq {}
impl<F: Field> Element for F {}

/// An index type of a leaf in a Merkle tree.
pub trait Index:
    Debug + Eq + PartialEq + Ord + PartialOrd + Clone + ToTraversalPath + IndexOps
{
}
impl Index for u64 {}

/// An internal node value type in a Merkle tree.
pub trait NodeValue: Default + Eq + PartialEq + Copy + Clone + Debug {}
impl<F: Field> NodeValue for F {}

/// Merkle tree hash function
pub trait DigestAlgorithm<E, I, T>
where
    E: Element,
    I: Index,
    T: NodeValue,
{
    // Possible improvement: adding digest_element() and digest_index()
    /// Digest a list of values
    fn digest(data: &[T]) -> T;

    /// Digest a leaf (an indexed element)
    fn digest_leaf(pos: &I, elem: &E) -> T;
}

/// Ops needs to be performed over index
pub trait IndexOps<Rhs = Self>: AddAssign<Rhs> {}

impl<T, Rhs> IndexOps<Rhs> for T where T: AddAssign<Rhs> {}

/// An trait for Merkle tree index type.
pub trait ToTraversalPath {
    /// Convert the given index to a vector of branch indices given tree height
    /// and arity.
    fn to_traverse_path(&self, height: usize, arity: usize) -> Vec<usize>;
}

impl ToTraversalPath for u64 {
    fn to_traverse_path(&self, height: usize, arity: usize) -> Vec<usize> {
        let mut pos = *self;
        let mut ret = vec![];
        for _i in 0..height {
            ret.push((pos % (arity as u64)) as usize);
            pos /= arity as u64;
        }
        ret
    }
}

impl ToTraversalPath for BigUint {
    fn to_traverse_path(&self, height: usize, arity: usize) -> Vec<usize> {
        let mut pos = self.clone();
        let mut ret = vec![];
        for _i in 0..height {
            ret.push((&pos % (arity as u64)).to_usize().unwrap());
            pos /= arity as u64;
        }
        ret
    }
}

/// A merkle commitment consists a root hash value, a tree height and number of
/// leaves
#[derive(Eq, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub struct MerkleCommitment<T: NodeValue> {
    /// Root of a tree
    pub root_value: T,
    /// Height of a tree
    pub height: usize,
    /// Number of leaves in the tree
    pub num_leaves: u64,
}

/// Basic functionalities for a merkle tree implementation. Abstracted as an
/// accumulator for fixed-length array. Supports generate membership proof at a
/// given position and verify a membership proof.
pub trait MerkleTreeScheme: Sized {
    /// Merkle tree element type
    type Element: Element;
    /// Hash algorithm used in merkle tree
    type Digest: DigestAlgorithm<Self::Element, Self::Index, Self::NodeValue>;
    /// Index type for this merkle tree
    type Index: Index;
    /// Internal and root node value
    type NodeValue: NodeValue;
    /// Merkle proof
    type MembershipProof: Clone;
    /// Batch proof
    type BatchMembershipProof: Clone;

    /// Tree arity
    const ARITY: usize;

    /// Construct a new merkle tree with given height from a data slice
    fn from_elems(
        height: usize,
        elems: impl IntoIterator<Item = impl Borrow<Self::Element>>,
    ) -> Result<Self, PrimitivesError>;

    /// Return the height of this merkle tree
    fn height(&self) -> usize;
    /// Return the maximum allowed number leaves
    fn capacity(&self) -> BigUint;
    /// Return the current number of leaves
    fn num_leaves(&self) -> u64;

    /// Return the current root value
    fn root(&self) -> Self::NodeValue;

    /// Return a merkle commitment
    fn commitment(&self) -> MerkleCommitment<Self::NodeValue>;

    /// Returns the leaf value given a position
    /// * `pos` - zero-based index of the leaf in the tree
    /// * `returns` - Leaf value at the position along with a proof.
    ///   LookupResult::EmptyLeaf if the leaf position is empty or invalid,
    ///   LookupResult::NotInMemory if the leaf position has been forgotten.
    fn lookup(
        &self,
        pos: impl Borrow<Self::Index>,
    ) -> LookupResult<Self::Element, Self::MembershipProof>;

    /// Verify an element is a leaf of a Merkle tree given the proof
    /// * `pos` - zero-based index of the leaf in the tree
    /// * `proof` - a merkle tree proof
    /// * `returns` - Ok(true) if the proof is accepted, Ok(false) if not. Err()
    ///   if the proof is not well structured, E.g. not for this merkle tree.
    fn verify(
        &self,
        pos: impl Borrow<Self::Index>,
        proof: impl Borrow<Self::MembershipProof>,
    ) -> Result<bool, PrimitivesError>;

    // fn batch_lookup(&self, pos: impl Iterator<Item = usize>) -> LookupResult<(),
    // Self::BatchProof>; fn batch_verify(
    //     &self,
    //     pos: impl Iterator<Item = usize>,
    //     proof: impl Borrow<Self::BatchProof>,
    // ) -> Result<(), PrimitivesError>;
}

/// Merkle tree that allows insertion at back. Abstracted as a commitment for
/// append-only vector.
pub trait AppendableMerkleTreeScheme: MerkleTreeScheme {
    /// Insert a new value at the leftmost available slot
    /// * `elem` - element to insert in the tree
    /// * `returns` - Ok(()) if successful
    fn push(&mut self, elem: impl Borrow<Self::Element>) -> Result<(), PrimitivesError>;

    /// Insert a list of new values at the leftmost available slots
    /// * `elems` - elements to insert
    /// * `returns` - Ok(()) if successful. If there are too many elements,
    ///   insertions will be performed until the merkle tree is full, and wil
    ///   return an Err().
    fn extend(
        &mut self,
        elems: impl IntoIterator<Item = impl Borrow<Self::Element>>,
    ) -> Result<(), PrimitivesError> {
        for elem in elems {
            self.push(elem)?;
        }
        Ok(())
    }
}

/// A universal merkle tree is abstracted as a random-access array or a
/// key-value map. It allows manipulation at any given position, and has ability
/// to generate/verify a non-membership proof.
pub trait UniversalMerkleTreeScheme: MerkleTreeScheme {
    /// Non membership proof for a given index
    type NonMembershipProof;
    /// Batch non membership proof
    type BatchNonMembershipProof;

    /// Update the leaf value at a given position
    /// * `pos` - zero-based index of the leaf in the tree
    /// * `elem` - newly updated element
    fn update(
        &mut self,
        pos: impl Borrow<Self::Index>,
        elem: impl Borrow<Self::Element>,
    ) -> LookupResult<Self::Element, ()>;

    /// Build a universal merkle tree from a key-value set.
    /// * `height` - height of the merkle tree
    /// * `data` - an iterator of key-value pairs. Could be a std hashmap or
    ///   simply an array or a slice of (key, value) pairs
    fn from_kv_set<BI, BE>(
        height: usize,
        data: impl IntoIterator<Item = impl Borrow<(BI, BE)>>,
    ) -> Result<Self, PrimitivesError>
    where
        BI: Borrow<Self::Index>,
        BE: Borrow<Self::Element>;
    // TODO(Chengyu): non-membership proof interfaces
}

/// Merkle tree that allows forget/remember elements from the memory
pub trait ForgetableMerkleTreeScheme: MerkleTreeScheme {
    /// Trim the leaf at position `i` from memory, if present.
    /// Should not trim if position `i` is the last inserted leaf position.
    /// Return is identical to result if `get_leaf(pos)` were called before this
    /// call.
    fn forget(&mut self, pos: Self::Index) -> LookupResult<Self::Element, Self::MembershipProof>;

    /// "Re-insert" a leaf into the tree using its proof.
    /// Returns Ok(()) if insertion is successful, or Err(err) if the
    /// proof disagrees with the merkle tree
    fn remember(
        &mut self,
        pos: Self::Index,
        element: impl Borrow<Self::Element>,
        proof: impl Borrow<Self::MembershipProof>,
    ) -> Result<(), PrimitivesError>;
}
