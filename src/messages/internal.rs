use serde::{Deserialize, Serialize};

use crate::{Error, Result};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DHTMessage {
    #[serde(rename = "t", with = "serde_bytes")]
    pub transaction_id: Vec<u8>,

    #[serde(default)]
    #[serde(rename = "v", with = "serde_bytes")]
    pub version: Option<Vec<u8>>,

    #[serde(flatten)]
    pub variant: DHTMessageVariant,

    #[serde(default)]
    #[serde(with = "serde_bytes")]
    pub ip: Option<Vec<u8>>,

    #[serde(default)]
    #[serde(rename = "ro")]
    pub read_only: Option<i32>,
}

impl DHTMessage {
    pub fn from_bytes<T: AsRef<[u8]>>(bytes: T) -> Result<DHTMessage> {
        let bytes = bytes.as_ref();
        let obj = serde_bencode::from_bytes(bytes)?;
        Ok(obj)
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        serde_bencode::to_bytes(self).map_err(Error::BencodeError)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "y")]
pub enum DHTMessageVariant {
    #[serde(rename = "q")]
    Request(DHTRequestSpecific),

    #[serde(rename = "r")]
    Response(DHTResponseSpecific),

    #[serde(rename = "e")]
    Error(DHTErrorSpecific),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "q")]
pub enum DHTRequestSpecific {
    #[serde(rename = "ping")]
    Ping {
        #[serde(rename = "a")]
        arguments: DHTPingRequestArguments,
    },

    #[serde(rename = "find_node")]
    FindNode {
        #[serde(rename = "a")]
        arguments: DHTFindNodeRequestArguments,
    },

    #[serde(rename = "get_peers")]
    GetPeers {
        #[serde(rename = "a")]
        arguments: DHTGetPeersRequestArguments,
    },

    #[serde(rename = "announce_peer")]
    AnnouncePeer {
        #[serde(rename = "a")]
        arguments: DHTAnnouncePeerRequestArguments,
    },

    #[serde(rename = "get_value")]
    GetValue {
        #[serde(rename = "a")]
        arguments: DHTGetValueArguments,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)] // This means order matters! Order these from most to least detailed
pub enum DHTResponseSpecific {
    GetValue {
        #[serde(rename = "r")]
        arguments: DHTGetValueResponseArguments,
    },

    GetPeers {
        #[serde(rename = "r")]
        arguments: DHTGetPeersResponseArguments,
    },

    FindNode {
        #[serde(rename = "r")]
        arguments: DHTFindNodeResponseArguments,
    },

    Ping {
        #[serde(rename = "r")]
        arguments: DHTPingResponseArguments,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DHTErrorSpecific {
    #[serde(rename = "e")]
    pub error_info: Vec<serde_bencode::value::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum DHTErrorValue {
    #[serde(rename = "")]
    ErrorCode(i32),
    ErrorDescription(String),
}

// === PING ===

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DHTPingRequestArguments {
    #[serde(with = "serde_bytes")]
    pub id: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DHTPingResponseArguments {
    #[serde(with = "serde_bytes")]
    pub id: Vec<u8>,
}

// === FIND NODE ===

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DHTFindNodeRequestArguments {
    #[serde(with = "serde_bytes")]
    pub id: Vec<u8>,

    #[serde(with = "serde_bytes")]
    pub target: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DHTFindNodeResponseArguments {
    #[serde(with = "serde_bytes")]
    pub id: Vec<u8>,

    #[serde(with = "serde_bytes")]
    pub nodes: Vec<u8>,
}

// === Get Peers ===

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DHTGetPeersRequestArguments {
    #[serde(with = "serde_bytes")]
    pub id: Vec<u8>,

    #[serde(with = "serde_bytes")]
    pub info_hash: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DHTGetPeersResponseArguments {
    #[serde(with = "serde_bytes")]
    pub id: Vec<u8>,

    #[serde(with = "serde_bytes")]
    pub token: Vec<u8>,

    #[serde(with = "serde_bytes")]
    #[serde(default)]
    pub nodes: Option<Vec<u8>>,

    #[serde(default)]
    pub values: Option<Vec<serde_bytes::ByteBuf>>,
}

// === Announce Peer ===

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DHTAnnouncePeerRequestArguments {
    #[serde(with = "serde_bytes")]
    pub id: Vec<u8>,

    #[serde(with = "serde_bytes")]
    pub info_hash: Vec<u8>,

    pub port: u16,

    #[serde(with = "serde_bytes")]
    pub token: Vec<u8>,

    #[serde(default)]
    pub implied_port: Option<u8>,
}

// === Get Value ===

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DHTGetValueArguments {
    #[serde(with = "serde_bytes")]
    pub id: Vec<u8>,

    #[serde(with = "serde_bytes")]
    pub target: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DHTGetValueResponseArguments {
    #[serde(with = "serde_bytes")]
    pub id: Vec<u8>,

    #[serde(with = "serde_bytes")]
    pub token: Vec<u8>,

    #[serde(with = "serde_bytes")]
    #[serde(default)]
    pub nodes: Option<Vec<u8>>,

    #[serde(with = "serde_bytes")]
    #[serde(default)]
    pub v: Vec<u8>,

    #[serde(default)]
    pub k: Option<Vec<u8>>,

    #[serde(with = "serde_bytes")]
    #[serde(default)]
    pub sig: Option<Vec<u8>>,

    #[serde(default)]
    pub seq: Option<i64>,
}
