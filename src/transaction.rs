// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::base_types::SuiAddress;
use crate::crypto::EmptySignInfo;
use crate::message_envelope::{Envelope, Message};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Simple digest type for minimal implementation
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Digest([u8; 32]);

impl Digest {
    pub fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }
}

/// Transaction digest
pub type TransactionDigest = Digest;

/// Object reference (simplified)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ObjectRef {
    pub id: ObjectID,
    pub version: SequenceNumber,
    pub digest: ObjectDigest,
}

/// Object ID (simplified)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ObjectID(SuiAddress);

impl ObjectID {
    pub const ZERO: Self = Self(SuiAddress::ZERO);
}

/// Sequence number (simplified)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SequenceNumber(u64);

impl Default for SequenceNumber {
    fn default() -> Self {
        Self(0)
    }
}

/// Object digest (simplified)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ObjectDigest(Digest);

impl ObjectDigest {
    pub const MIN: Self = Self(Digest([0u8; 32]));
}

/// Transaction kind (simplified)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TransactionKind {
    ProgrammableTransaction(ProgrammableTransaction),
}

/// Programmable transaction (simplified)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProgrammableTransaction {
    pub inputs: Vec<CallArg>,
    pub commands: Vec<Command>,
}

/// Call argument (simplified)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CallArg {
    Pure(Vec<u8>),
    Object(ObjectArg),
}

/// Object argument (simplified)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ObjectArg {
    ImmOrOwned(ObjectRef),
    Shared { id: ObjectID },
}

/// Command (simplified)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Command {
    MoveCall(MoveCall),
}

/// Move call (simplified)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MoveCall {
    pub package: ObjectID,
    pub module: String,
    pub function: String,
    pub type_arguments: Vec<TypeTag>,
    pub arguments: Vec<CallArg>,
}

/// Type tag (simplified)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TypeTag {
    Bool,
    U8,
    U64,
    U128,
    Address,
    Vector(Box<TypeTag>),
    Struct(StructTag),
}

/// Struct tag (simplified)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StructTag {
    pub address: SuiAddress,
    pub module: String,
    pub name: String,
    pub type_params: Vec<TypeTag>,
}

/// Gas data (simplified)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GasData {
    pub payment: Vec<ObjectRef>,
    pub owner: SuiAddress,
    pub price: u64,
    pub budget: u64,
}

/// Transaction expiration (simplified)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TransactionExpiration {
    None,
    Epoch(u64),
}

/// Transaction data (simplified)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TransactionData {
    pub kind: TransactionKind,
    pub sender: SuiAddress,
    pub gas_data: GasData,
    pub expiration: TransactionExpiration,
}

impl TransactionData {
    pub fn new(
        kind: TransactionKind,
        sender: SuiAddress,
        gas_payment: ObjectRef,
        gas_budget: u64,
        gas_price: u64,
    ) -> Self {
        Self {
            kind,
            sender,
            gas_data: GasData {
                price: gas_price,
                owner: sender,
                payment: vec![gas_payment],
                budget: gas_budget,
            },
            expiration: TransactionExpiration::None,
        }
    }
}

/// Sender signed transaction (simplified)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SenderSignedTransaction {
    pub intent_message: IntentMessage<TransactionData>,
}

/// Intent message (simplified)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct IntentMessage<T> {
    pub intent: Intent,
    pub value: T,
}

/// Intent (simplified)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Intent {
    pub version: u8,
    pub scope: IntentScope,
}

/// Intent scope (simplified)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IntentScope {
    TransactionData,
}

/// Sender signed data (simplified)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SenderSignedData {
    pub transactions: Vec<SenderSignedTransaction>,
}

impl Message for SenderSignedData {
    type DigestType = TransactionDigest;

    fn digest(&self) -> Self::DigestType {
        // Simplified digest for minimal implementation
        TransactionDigest::new([0u8; 32])
    }
}

/// Transaction type alias
pub type Transaction = Envelope<SenderSignedData, EmptySignInfo>;

impl fmt::Display for TransactionData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TransactionData {{ sender: {}, gas_budget: {} }}", 
               self.sender, self.gas_data.budget)
    }
}
