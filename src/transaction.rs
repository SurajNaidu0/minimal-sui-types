// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::base_types::SuiAddress;
use crate::crypto::{EmptySignInfo, SuiSignature};
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

    /// Create a new digest from bytes
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut digest = [0u8; 32];
        let len = std::cmp::min(bytes.len(), 32);
        digest[..len].copy_from_slice(&bytes[..len]);
        Self(digest)
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

impl ObjectRef {
    /// Create a new object reference
    pub fn new(id: ObjectID, version: SequenceNumber, digest: ObjectDigest) -> Self {
        Self { id, version, digest }
    }
}

/// Object ID (simplified)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ObjectID(SuiAddress);

impl ObjectID {
    pub const ZERO: Self = Self(SuiAddress::ZERO);

    /// Create a new object ID
    pub fn new(address: SuiAddress) -> Self {
        Self(address)
    }
}

/// Sequence number (simplified)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SequenceNumber(u64);

impl Default for SequenceNumber {
    fn default() -> Self {
        Self(0)
    }
}

impl SequenceNumber {
    /// Create a new sequence number
    pub fn new(value: u64) -> Self {
        Self(value)
    }
}

/// Object digest (simplified)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ObjectDigest(Digest);

impl ObjectDigest {
    pub const MIN: Self = Self(Digest([0u8; 32]));

    /// Create a new object digest
    pub fn new(digest: Digest) -> Self {
        Self(digest)
    }
}

/// Transaction kind (simplified)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TransactionKind {
    ProgrammableTransaction(ProgrammableTransaction),
}

impl TransactionKind {
    /// Create a new programmable transaction
    pub fn new(pt: ProgrammableTransaction) -> Self {
        Self::ProgrammableTransaction(pt)
    }
}

/// Programmable transaction (simplified)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProgrammableTransaction {
    pub inputs: Vec<CallArg>,
    pub commands: Vec<Command>,
}

impl ProgrammableTransaction {
    /// Create a new programmable transaction
    pub fn new(inputs: Vec<CallArg>, commands: Vec<Command>) -> Self {
        Self { inputs, commands }
    }
}

/// Call argument (simplified)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CallArg {
    Pure(Vec<u8>),
    Object(ObjectArg),
}

impl CallArg {
    /// Create a new pure call argument
    pub fn new_pure(data: Vec<u8>) -> Self {
        Self::Pure(data)
    }

    /// Create a new object call argument
    pub fn new_object(object_arg: ObjectArg) -> Self {
        Self::Object(object_arg)
    }
}

/// Object argument (simplified)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ObjectArg {
    ImmOrOwned(ObjectRef),
    Shared { id: ObjectID },
}

impl ObjectArg {
    /// Create a new immutable or owned object argument
    pub fn new_imm_or_owned(object_ref: ObjectRef) -> Self {
        Self::ImmOrOwned(object_ref)
    }

    /// Create a new shared object argument
    pub fn new_shared(id: ObjectID) -> Self {
        Self::Shared { id }
    }
}

/// Command (simplified)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Command {
    MoveCall(MoveCall),
}

impl Command {
    /// Create a new move call command
    pub fn new_move_call(move_call: MoveCall) -> Self {
        Self::MoveCall(move_call)
    }
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

impl MoveCall {
    /// Create a new move call
    pub fn new(
        package: ObjectID,
        module: String,
        function: String,
        type_arguments: Vec<TypeTag>,
        arguments: Vec<CallArg>,
    ) -> Self {
        Self {
            package,
            module,
            function,
            type_arguments,
            arguments,
        }
    }

    /// Create a new move call with string references
    pub fn new_with_str(
        package: ObjectID,
        module: &str,
        function: &str,
        type_arguments: Vec<TypeTag>,
        arguments: Vec<CallArg>,
    ) -> Self {
        Self {
            package,
            module: module.to_string(),
            function: function.to_string(),
            type_arguments,
            arguments,
        }
    }
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

impl TypeTag {
    /// Create a new bool type tag
    pub fn new_bool() -> Self {
        Self::Bool
    }

    /// Create a new u8 type tag
    pub fn new_u8() -> Self {
        Self::U8
    }

    /// Create a new u64 type tag
    pub fn new_u64() -> Self {
        Self::U64
    }

    /// Create a new u128 type tag
    pub fn new_u128() -> Self {
        Self::U128
    }

    /// Create a new address type tag
    pub fn new_address() -> Self {
        Self::Address
    }

    /// Create a new vector type tag
    pub fn new_vector(inner: TypeTag) -> Self {
        Self::Vector(Box::new(inner))
    }

    /// Create a new struct type tag
    pub fn new_struct(struct_tag: StructTag) -> Self {
        Self::Struct(struct_tag)
    }
}

/// Struct tag (simplified)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StructTag {
    pub address: SuiAddress,
    pub module: String,
    pub name: String,
    pub type_params: Vec<TypeTag>,
}

impl StructTag {
    /// Create a new struct tag
    pub fn new(
        address: SuiAddress,
        module: String,
        name: String,
        type_params: Vec<TypeTag>,
    ) -> Self {
        Self {
            address,
            module,
            name,
            type_params,
        }
    }
}

/// Gas data (simplified)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GasData {
    pub payment: Vec<ObjectRef>,
    pub owner: SuiAddress,
    pub price: u64,
    pub budget: u64,
}

impl GasData {
    /// Create a new gas data
    pub fn new(payment: Vec<ObjectRef>, owner: SuiAddress, price: u64, budget: u64) -> Self {
        Self {
            payment,
            owner,
            price,
            budget,
        }
    }
}

/// Transaction expiration (simplified)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TransactionExpiration {
    None,
    Epoch(u64),
}

impl TransactionExpiration {
    /// Create a new none expiration
    pub fn new_none() -> Self {
        Self::None
    }

    /// Create a new epoch expiration
    pub fn new_epoch(epoch: u64) -> Self {
        Self::Epoch(epoch)
    }
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

    pub fn new_with_gas_coins(
        kind: TransactionKind,
        sender: SuiAddress,
        gas_payment: Vec<ObjectRef>,
        gas_budget: u64,
        gas_price: u64,
    ) -> Self {
        Self {
            kind,
            sender,
            gas_data: GasData {
                price: gas_price,
                owner: sender,
                payment: gas_payment,
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

impl SenderSignedTransaction {
    /// Create a new sender signed transaction
    pub fn new(intent_message: IntentMessage<TransactionData>) -> Self {
        Self { intent_message }
    }
}

/// Intent message (simplified)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct IntentMessage<T> {
    pub intent: Intent,
    pub value: T,
}

impl<T> IntentMessage<T> {
    /// Create a new intent message
    pub fn new(intent: Intent, value: T) -> Self {
        Self { intent, value }
    }
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
    ProofOfPossession,
}

impl IntentScope {
    /// Create a sui app intent scope
    pub fn sui_app(scope: IntentScope) -> IntentScope {
        scope
    }
}

impl Intent {
    /// Create a new intent with default values
    pub fn new(scope: IntentScope) -> Self {
        Self {
            version: 0,
            scope,
        }
    }
    
    /// Create a new intent with custom version
    pub fn with_version(version: u8, scope: IntentScope) -> Self {
        Self { version, scope }
    }

    /// Create a sui app intent
    pub fn sui_app(scope: IntentScope) -> Self {
        Self {
            version: 0,
            scope,
        }
    }
}

/// Sender signed data (simplified)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SenderSignedData {
    pub transactions: Vec<SenderSignedTransaction>,
}

impl SenderSignedData {
    /// Create a new sender signed data
    pub fn new(transactions: Vec<SenderSignedTransaction>) -> Self {
        Self { transactions }
    }
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

impl Transaction {
    /// Create a new transaction from data and signatures
    pub fn from_data(data: SenderSignedData, _signatures: Vec<SuiSignature>) -> Self {
        // For minimal implementation, we'll use EmptySignInfo
        // In a real implementation, this would create proper signature info
        Envelope::new_from_data_and_sig(data, EmptySignInfo {})
    }
}

impl fmt::Display for TransactionData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TransactionData {{ sender: {}, gas_budget: {} }}", 
               self.sender, self.gas_data.budget)
    }
}
