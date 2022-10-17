// Copyright (c) 2022 Espresso Systems (espressosys.com)
// This file is part of the Jellyfish library.

// You should have received a copy of the MIT License
// along with the Jellyfish library. If not, see <https://mit-license.org/>.

use crate::errors::PrimitivesError;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize, Read, SerializationError, Write};
use ark_std::{
    borrow::Borrow,
    fmt::{Debug, Display},
    ops::{Add, AddAssign, DivAssign, MulAssign, Rem},
    string::ToString,
    vec,
    vec::Vec,
};
use num_traits::AsPrimitive;
use serde::{Deserialize, Serialize};
// use to_vec::ToVec;

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

/// Merkle tree hash function
pub trait DigestAlgorithm<T> {
    /// Digest a list of values
    fn digest(data: &[T]) -> T;
}

/// Ops needs to be performed over index
pub trait IndexOps<Rhs = Self>:
    AddAssign<Rhs> + Add<Rhs, Output = Self> + DivAssign<Rhs> + MulAssign<Rhs> + Rem<Rhs, Output = Self>
{
}

impl<T, Rhs> IndexOps<Rhs> for T where
    T: AddAssign<Rhs>
        + Add<Rhs, Output = Self>
        + DivAssign<Rhs>
        + MulAssign<Rhs>
        + Rem<Rhs, Output = Self>
{
}

/// Convert into a vector of T
pub trait ToVec<T> {
    /// Return a vector of T
    fn to_vec(&self) -> Vec<T>;
}

impl<T: Copy> ToVec<T> for T {
    fn to_vec(&self) -> Vec<T> {
        vec![*self]
    }
}

/// A merkle commitment consists a root hash value, a tree height and number of
/// leaves
#[derive(
    Eq, PartialEq, Clone, Copy, CanonicalSerialize, CanonicalDeserialize, Serialize, Deserialize,
)]
pub struct MerkleCommitment<
    I: CanonicalSerialize + CanonicalDeserialize,
    T: CanonicalSerialize + CanonicalDeserialize,
> {
    /// Root of a tree
    pub root_value: T,
    /// Height of a tree
    pub height: usize,
    /// Number of leaves in the tree
    pub num_leaves: I,
}

/// Basic functionalities for a merkle tree implementation. Abstracted as an
/// accumulator for fixed-length array. Supports generate membership proof at a
/// given position and verify a membership proof.
pub trait MerkleTree: Sized {
    /// Merkle tree element type
    type ElementType: Clone
        + Copy
        + CanonicalSerialize
        + CanonicalDeserialize
        + Eq
        + PartialEq
        + ToVec<Self::NodeValue>;
    /// Hash algorithm used in merkle tree
    type Digest: DigestAlgorithm<Self::NodeValue>;
    /// Index type for this merkle tree
    type IndexType: ToVec<Self::NodeValue>
        + Debug
        + Eq
        + PartialEq
        + Ord
        + PartialOrd
        + IndexOps
        + Clone
        + Copy
        + AsPrimitive<usize>
        + CanonicalDeserialize
        + CanonicalSerialize;
    /// Internal and root node value
    type NodeValue: Default
        + Eq
        + PartialEq
        + Copy
        + Clone
        + Display
        + CanonicalSerialize
        + CanonicalDeserialize;
    /// Merkle proof
    type MembershipProof;
    /// Batch proof
    type BatchMembershipProof;

    /// Tree arity
    const ARITY: usize;

    /// Construct a new merkle tree with given height from a data slice
    fn from_elems(
        height: usize,
        elems: impl IntoIterator<Item = impl Borrow<Self::ElementType>>,
    ) -> Result<Self, PrimitivesError>;

    /// Return the height of this merkle tree
    fn height(&self) -> usize;
    /// Return the maximum allowed number leaves
    fn capacity(&self) -> Self::IndexType;
    /// Return the current number of leaves
    fn num_leaves(&self) -> Self::IndexType;

    /// Return the current root value
    fn root(&self) -> Self::NodeValue;

    /// Return a merkle commitment
    fn commitment(&self) -> MerkleCommitment<Self::IndexType, Self::NodeValue>;

    /// Returns the leaf value given a position
    /// * `pos` - zero-based index of the leaf in the tree
    /// * `returns` - Leaf value at the position along with a proof.
    ///   LookupResult::EmptyLeaf if the leaf position is empty or invalid,
    ///   LookupResult::NotInMemory if the leaf position has been forgotten.
    fn lookup(
        &self,
        pos: Self::IndexType,
    ) -> LookupResult<Self::ElementType, Self::MembershipProof>;

    /// Verify an element is a leaf of a Merkle tree given the proof
    /// * `pos` - zero-based index of the leaf in the tree
    /// * `proof` - a merkle tree proof
    /// * `returns` - Ok(true) if the proof is accepted, Ok(false) if not. Err()
    ///   if the proof is not well structured, E.g. not for this merkle tree.
    fn verify(
        &self,
        pos: Self::IndexType,
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
pub trait AppendableMerkleTree: MerkleTree {
    /// Insert a new value at the leftmost available slot
    /// * `elem` - element to insert in the tree
    /// * `returns` - Ok(()) if successful
    fn push(&mut self, elem: impl Borrow<Self::ElementType>) -> Result<(), PrimitivesError>;

    /// Insert a list of new values at the leftmost available slots
    /// * `elems` - elements to insert
    /// * `returns` - Ok(()) if successful. If there are too many elements,
    ///   insertions will be performed until the merkle tree is full, and wil
    ///   return an Err().
    fn extend(
        &mut self,
        elems: impl IntoIterator<Item = impl Borrow<Self::ElementType>>,
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
pub trait UniversalMerkleTree: MerkleTree {
    /// Non membership proof for a given index
    type NonMembershipProof;
    /// Batch non membership proof
    type BatchNonMembershipProof;

    /// Update the leaf value at a given position
    /// * `pos` - zero-based index of the leaf in the tree
    /// * `elem` - newly updated element
    fn update(
        &mut self,
        pos: Self::IndexType,
        elem: &Self::ElementType,
    ) -> Result<(), PrimitivesError>;

    // TODO(Chengyu): non-membership proof interfaces
}

/// Merkle tree that allows forget/remember elements from the memory
pub trait ForgetableMerkleTree: MerkleTree {
    /// Trim the leaf at position `i` from memory, if present.
    /// Should not trim if position `i` is the last inserted leaf position.
    /// Return is identical to result if `get_leaf(pos)` were called before this
    /// call.
    fn forget(
        &mut self,
        pos: Self::IndexType,
    ) -> LookupResult<Self::ElementType, Self::MembershipProof>;

    /// "Re-insert" a leaf into the tree using its proof.
    /// Returns Ok(()) if insertion is successful, or Err(err) if the
    /// proof disagrees with the merkle tree
    fn remember(
        &mut self,
        pos: Self::IndexType,
        element: impl Borrow<Self::ElementType>,
        proof: impl Borrow<Self::MembershipProof>,
    ) -> Result<(), PrimitivesError>;
}
