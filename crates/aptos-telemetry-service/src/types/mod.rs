// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

pub mod auth;
pub mod telemetry;
pub mod validator_set;

pub mod common {

    use std::fmt;

    use crate::types::auth::Claims;
    use aptos_types::chain_id::ChainId;
    use aptos_types::PeerId;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct EventIdentity {
        pub peer_id: PeerId,
        pub chain_id: ChainId,
        pub role_type: NodeType,
        pub epoch: u64,
    }

    impl From<Claims> for EventIdentity {
        fn from(claims: Claims) -> Self {
            Self {
                peer_id: claims.peer_id,
                chain_id: claims.chain_id,
                role_type: claims.node_type,
                epoch: claims.epoch,
            }
        }
    }

    #[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
    pub enum NodeType {
        Validator,
        ValidatorFullNode,
        PublicFullNode,
        Unknown,
    }

    impl NodeType {
        pub fn as_str(self) -> &'static str {
            match self {
                NodeType::Validator => "validator",
                NodeType::ValidatorFullNode => "validator_fullnode",
                NodeType::PublicFullNode => "public_fullnode",
                NodeType::Unknown => "unknown_peer",
            }
        }
    }

    impl fmt::Debug for NodeType {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self)
        }
    }

    impl fmt::Display for NodeType {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.as_str())
        }
    }
}

pub mod humio {
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    #[derive(Deserialize, Serialize, Clone, Debug)]
    pub struct UnstructuredLog {
        pub fields: HashMap<String, String>,
        pub tags: HashMap<String, String>,
        pub messages: Vec<String>,
    }
}
