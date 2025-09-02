// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};

/// Empty signature info for unsigned transactions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmptySignInfo {}

/// SuiKeyPair represents a key pair for signing transactions
#[derive(Debug, Clone)]
pub enum SuiKeyPair {
    Ed25519(Vec<u8>),
    Secp256k1(Vec<u8>),
    Secp256r1(Vec<u8>),
}

impl SuiKeyPair {
    pub fn public(&self) -> PublicKey {
        match self {
            SuiKeyPair::Ed25519(_) => PublicKey::Ed25519([0u8; 32]),
            SuiKeyPair::Secp256k1(_) => PublicKey::Secp256k1([0u8; 33]),
            SuiKeyPair::Secp256r1(_) => PublicKey::Secp256r1([0u8; 33]),
        }
    }

    pub fn copy(&self) -> Self {
        match self {
            SuiKeyPair::Ed25519(kp) => SuiKeyPair::Ed25519(kp.clone()),
            SuiKeyPair::Secp256k1(kp) => SuiKeyPair::Secp256k1(kp.clone()),
            SuiKeyPair::Secp256r1(kp) => SuiKeyPair::Secp256r1(kp.clone()),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.push(self.public().flag());

        match self {
            SuiKeyPair::Ed25519(kp) => {
                bytes.extend_from_slice(kp);
            }
            SuiKeyPair::Secp256k1(kp) => {
                bytes.extend_from_slice(kp);
            }
            SuiKeyPair::Secp256r1(kp) => {
                bytes.extend_from_slice(kp);
            }
        }
        bytes
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, eyre::Report> {
        match SignatureScheme::from_flag_byte(bytes.first().ok_or_else(|| eyre::eyre!("Invalid length"))?)
        {
            Ok(x) => match x {
                SignatureScheme::ED25519 => Ok(SuiKeyPair::Ed25519(
                    bytes.get(1..).ok_or_else(|| eyre::eyre!("Invalid length"))?.to_vec(),
                )),
                SignatureScheme::Secp256k1 => {
                    Ok(SuiKeyPair::Secp256k1(
                        bytes.get(1..).ok_or_else(|| eyre::eyre!("Invalid length"))?.to_vec(),
                    ))
                }
                SignatureScheme::Secp256r1 => {
                    Ok(SuiKeyPair::Secp256r1(
                        bytes.get(1..).ok_or_else(|| eyre::eyre!("Invalid length"))?.to_vec(),
                    ))
                }
            },
            _ => Err(eyre::eyre!("Invalid bytes")),
        }
    }
}

/// Public key types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PublicKey {
    Ed25519([u8; 32]),
    Secp256k1([u8; 33]),
    Secp256r1([u8; 33]),
}

impl PublicKey {
    pub fn flag(&self) -> u8 {
        match self {
            PublicKey::Ed25519(_) => SignatureScheme::ED25519.flag(),
            PublicKey::Secp256k1(_) => SignatureScheme::Secp256k1.flag(),
            PublicKey::Secp256r1(_) => SignatureScheme::Secp256r1.flag(),
        }
    }
}

/// Signature schemes
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SignatureScheme {
    ED25519,
    Secp256k1,
    Secp256r1,
}

impl SignatureScheme {
    pub fn flag(&self) -> u8 {
        match self {
            SignatureScheme::ED25519 => 0x00,
            SignatureScheme::Secp256k1 => 0x01,
            SignatureScheme::Secp256r1 => 0x02,
        }
    }

    pub fn from_flag_byte(flag: &u8) -> Result<Self, eyre::Report> {
        match flag {
            0x00 => Ok(SignatureScheme::ED25519),
            0x01 => Ok(SignatureScheme::Secp256k1),
            0x02 => Ok(SignatureScheme::Secp256r1),
            _ => Err(eyre::eyre!("Invalid signature scheme flag")),
        }
    }
}

/// Signature trait
pub trait Signature: Clone + Send + Sync {
    fn verify(&self, msg: &[u8], pk: &PublicKey) -> bool;
}

/// Basic signature implementation
#[derive(Debug, Clone)]
pub struct BasicSignature {
    pub scheme: SignatureScheme,
    pub signature_bytes: Vec<u8>,
}

impl Signature for BasicSignature {
    fn verify(&self, _msg: &[u8], _pk: &PublicKey) -> bool {
        // Simplified verification for minimal implementation
        true
    }
}
