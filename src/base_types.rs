// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};
use std::fmt;

/// SuiAddress is a 32-byte account address.
#[derive(
    Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash, Default, Debug, Serialize, Deserialize,
)]
pub struct SuiAddress([u8; 32]);

impl SuiAddress {
    pub const ZERO: Self = Self([0u8; 32]);

    pub fn new(address: [u8; 32]) -> Self {
        Self(address)
    }

    pub fn inner(&self) -> &[u8; 32] {
        &self.0
    }

    pub fn into_inner(self) -> [u8; 32] {
        self.0
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_vec()
    }

    pub fn to_hex_literal(&self) -> String {
        format!("0x{}", hex::encode(self.0))
    }

    pub fn to_string(&self) -> String {
        self.to_hex_literal()
    }
}

impl From<[u8; 32]> for SuiAddress {
    fn from(address: [u8; 32]) -> Self {
        Self(address)
    }
}

impl From<SuiAddress> for [u8; 32] {
    fn from(address: SuiAddress) -> Self {
        address.0
    }
}

impl From<&crate::crypto::PublicKey> for SuiAddress {
    fn from(public_key: &crate::crypto::PublicKey) -> Self {
        match public_key {
            crate::crypto::PublicKey::Ed25519(bytes) => {
                // Take first 32 bytes for address
                let mut address = [0u8; 32];
                let len = std::cmp::min(bytes.len(), 32);
                address[..len].copy_from_slice(&bytes[..len]);
                SuiAddress(address)
            }
            crate::crypto::PublicKey::Secp256k1(bytes) => {
                // Take first 32 bytes for address
                let mut address = [0u8; 32];
                let len = std::cmp::min(bytes.len(), 32);
                address[..len].copy_from_slice(&bytes[..len]);
                SuiAddress(address)
            }
            crate::crypto::PublicKey::Secp256r1(bytes) => {
                // Take first 32 bytes for address
                let mut address = [0u8; 32];
                let len = std::cmp::min(bytes.len(), 32);
                address[..len].copy_from_slice(&bytes[..len]);
                SuiAddress(address)
            }
        }
    }
}

impl AsRef<[u8]> for SuiAddress {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl fmt::Display for SuiAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_hex_literal())
    }
}

impl FromStr for SuiAddress {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix("0x").unwrap_or(s);
        let bytes = hex::decode(s)?;
        if bytes.len() != 32 {
            return Err(anyhow::anyhow!("Address must be 32 bytes"));
        }
        let mut address = [0u8; 32];
        address.copy_from_slice(&bytes);
        Ok(SuiAddress(address))
    }
}

use std::str::FromStr;
