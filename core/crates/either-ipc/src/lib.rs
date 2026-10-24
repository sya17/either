// SPDX-License-Identifier: GPL-3.0-or-later
//! The IPC contract of either (B2): JSON-RPC 2.0 over newline-delimited JSON on a Unix socket.
//!
//! Every type that crosses the socket is defined here and only here. Its JSON Schema is generated
//! from these types and kept as a snapshot in `tests/snapshots/`, so a change to the wire format
//! fails the tests until the snapshot is updated on purpose.
//!
//! Compatibility: within one [`PROTO`], only optional fields, methods and topics are added.
//! Readers ignore fields they do not know, so no type here denies unknown fields.

mod frame;

use std::collections::BTreeMap;

use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

pub use frame::{FrameError, LineDecoder, MAX_LINE_BYTES, encode_line};

/// Major version of the contract, sent in `hello`.
pub const PROTO: u32 = 1;

/// Version of the event object, the `v` in `{v, seq, ts, topic, data}`.
pub const EVENT_VERSION: u32 = 1;

/// Method names of proto 1.
pub mod method {
    pub const HELLO: &str = "hello";
    pub const SUBSCRIBE: &str = "subscribe";
    pub const UNSUBSCRIBE: &str = "unsubscribe";
    pub const STATE_SNAPSHOT: &str = "state.snapshot";
    /// The notification the daemon uses to deliver an [`Event`](crate::Event).
    pub const EVENT: &str = "event";
}

/// The `"jsonrpc": "2.0"` member. Any other value is rejected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum JsonRpc {
    #[serde(rename = "2.0")]
    V2,
}

/// A request id: a number or a string, as JSON-RPC 2.0 allows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum Id {
    Number(i64),
    String(String),
}

/// A call that expects a response with the same `id`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Request {
    pub jsonrpc: JsonRpc,
    pub id: Id,
    pub method: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub params: Option<Value>,
}

/// A message without an `id`; it gets no response. The daemon delivers events this way.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Notification {
    pub jsonrpc: JsonRpc,
    pub method: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub params: Option<Value>,
}

/// The answer to a request, with the same `id`. `id` is null only when the request could not be
/// read.
#[derive(Debug, Clone, PartialEq, Serialize, JsonSchema)]
pub struct Response {
    pub jsonrpc: JsonRpc,
    pub id: Option<Id>,
    #[serde(flatten)]
    pub outcome: Outcome,
}

/// A response before the check that it holds exactly one outcome; `flatten` alone would accept
/// both and keep whichever comes first.
#[derive(Deserialize)]
struct RawResponse {
    jsonrpc: JsonRpc,
    id: Option<Id>,
    #[serde(default, deserialize_with = "present")]
    result: Option<Value>,
    #[serde(default)]
    error: Option<ErrorObject>,
}

/// Keeps `"result": null`, a valid result, apart from a missing `result`.
fn present<'de, D: Deserializer<'de>>(value: D) -> Result<Option<Value>, D::Error> {
    Value::deserialize(value).map(Some)
}

// Written by hand instead of `#[serde(try_from)]`, which would also make schemars describe the
// lenient `RawResponse` instead of the one-outcome shape.
impl<'de> Deserialize<'de> for Response {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let raw = RawResponse::deserialize(deserializer)?;
        let outcome = match (raw.result, raw.error) {
            (Some(result), None) => Outcome::Result(result),
            (None, Some(error)) => Outcome::Error(error),
            _ => {
                return Err(serde::de::Error::custom(
                    "a response has exactly one of `result` or `error`",
                ));
            }
        };
        Ok(Self {
            jsonrpc: raw.jsonrpc,
            id: raw.id,
            outcome,
        })
    }
}

/// Exactly one of `result` or `error`.
#[derive(Debug, Clone, PartialEq, Serialize, JsonSchema)]
pub enum Outcome {
    #[serde(rename = "result")]
    Result(Value),
    #[serde(rename = "error")]
    Error(ErrorObject),
}

/// A JSON-RPC error. `code` stays a plain integer so a client can read codes added later.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ErrorObject {
    pub code: i32,
    /// For the codes of proto 1, the error name in snake_case, such as `version_mismatch`.
    pub message: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

/// The error codes of proto 1.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCode {
    ParseError,
    InvalidRequest,
    MethodNotFound,
    InvalidParams,
    /// The major `proto` of the client is not supported.
    VersionMismatch,
    /// The client may not call this method.
    Unauthorized,
    /// A bridge (Hyprland, D-Bus) is not connected yet.
    NotReady,
    RateLimited,
}

impl ErrorCode {
    pub const ALL: [Self; 8] = [
        Self::ParseError,
        Self::InvalidRequest,
        Self::MethodNotFound,
        Self::InvalidParams,
        Self::VersionMismatch,
        Self::Unauthorized,
        Self::NotReady,
        Self::RateLimited,
    ];

    pub const fn code(self) -> i32 {
        match self {
            Self::ParseError => -32700,
            Self::InvalidRequest => -32600,
            Self::MethodNotFound => -32601,
            Self::InvalidParams => -32602,
            Self::VersionMismatch => -32001,
            Self::Unauthorized => -32002,
            Self::NotReady => -32003,
            Self::RateLimited => -32004,
        }
    }

    pub const fn name(self) -> &'static str {
        match self {
            Self::ParseError => "parse_error",
            Self::InvalidRequest => "invalid_request",
            Self::MethodNotFound => "method_not_found",
            Self::InvalidParams => "invalid_params",
            Self::VersionMismatch => "version_mismatch",
            Self::Unauthorized => "unauthorized",
            Self::NotReady => "not_ready",
            Self::RateLimited => "rate_limited",
        }
    }
}

impl From<ErrorCode> for ErrorObject {
    fn from(code: ErrorCode) -> Self {
        Self {
            code: code.code(),
            message: code.name().to_owned(),
            data: None,
        }
    }
}

/// Params of `hello`, which must be the first message on a connection.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct HelloParams {
    pub proto: u32,
    /// What is connecting, for example `shell` or `cli`.
    pub client: String,
    /// Version of the client package.
    pub version: String,
}

/// Result of `hello`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct HelloResult {
    pub proto: u32,
    /// Version of the daemon package.
    pub daemon: String,
    /// The latest event `seq` at the time of the answer.
    pub seq: u64,
}

/// Params of `subscribe` and `unsubscribe`: topic globs such as `hypr.*`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct TopicsParams {
    pub topics: Vec<String>,
}

/// Result of `subscribe` and `unsubscribe`. An object, so fields can be added later.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct EmptyResult {}

/// Result of `state.snapshot`: the full state of each domain and the `seq` it reflects.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct SnapshotResult {
    pub seq: u64,
    pub domains: BTreeMap<String, Value>,
}

/// An event, delivered as the params of an `event` notification.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Event {
    /// Version of the event format: 1 in proto 1.
    pub v: u32,
    /// Grows with every event the daemon publishes.
    pub seq: u64,
    /// Milliseconds since the Unix epoch.
    pub ts: u64,
    /// `<domain>.<object>.<event>`, for example `hypr.workspace.focused`.
    pub topic: String,
    pub data: Value,
}
