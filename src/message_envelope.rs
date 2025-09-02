// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};

/// Message trait that defines the basic interface for messages
pub trait Message {
    type DigestType: Clone + Debug;
    
    fn digest(&self) -> Self::DigestType;
}

/// Envelope wraps a message with authentication information
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Envelope<T: Message + PartialEq + Eq, S: PartialEq + Eq> {
    data: T,
    auth_signature: S,
}

impl<T: Message + PartialEq + Eq, S: PartialEq + Eq> Envelope<T, S> {
    pub fn new_from_data_and_sig(data: T, sig: S) -> Self {
        Self {
            data,
            auth_signature: sig,
        }
    }

    pub fn data(&self) -> &T {
        &self.data
    }

    pub fn into_data(self) -> T {
        self.data
    }

    pub fn into_sig(self) -> S {
        self.auth_signature
    }

    pub fn into_data_and_sig(self) -> (T, S) {
        (self.data, self.auth_signature)
    }

    pub fn auth_signature(&self) -> &S {
        &self.auth_signature
    }
}

impl<T: Message + Display + PartialEq + Eq, S: PartialEq + Eq> Display for Envelope<T, S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data)
    }
}
