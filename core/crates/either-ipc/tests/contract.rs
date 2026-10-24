// SPDX-License-Identifier: GPL-3.0-or-later
//! The wire format of proto 1, checked from the outside.

use std::collections::BTreeMap;

use either_ipc::*;
use serde::de::DeserializeOwned;
use serde_json::{Value, json};

/// The example session from the architecture documentation, one message per line.
const EXAMPLE_HELLO: &str = r#"{"jsonrpc":"2.0","id":1,"method":"hello","params":{"proto":1,"client":"shell","version":"0.1.0"}}"#;
const EXAMPLE_HELLO_RESULT: &str =
    r#"{"jsonrpc":"2.0","id":1,"result":{"proto":1,"daemon":"0.1.0","seq":4812}}"#;
const EXAMPLE_SUBSCRIBE: &str = r#"{"jsonrpc":"2.0","id":2,"method":"subscribe","params":{"topics":["hypr.*","media.player.*"]}}"#;
const EXAMPLE_EVENT: &str = r#"{"jsonrpc":"2.0","method":"event","params":{"v":1,"seq":4813,"ts":1760000000123,"topic":"hypr.workspace.focused","data":{"id":3,"monitor":"DP-1"}}}"#;
const EXAMPLE_DISPATCH: &str = r#"{"jsonrpc":"2.0","id":3,"method":"hypr.dispatch","params":{"dispatcher":"workspace","arg":"4"}}"#;
const EXAMPLE_UNAUTHORIZED: &str = r#"{"jsonrpc":"2.0","id":3,"error":{"code":-32002,"message":"unauthorized","data":{"method":"hypr.dispatch"}}}"#;

/// Parses `line` as `T`, serializes it again, and checks that nothing was lost or added.
fn round_trip<T: DeserializeOwned + serde::Serialize>(line: &str) -> T {
    let parsed: T = serde_json::from_str(line).expect("example parses");
    let original: Value = serde_json::from_str(line).expect("example is JSON");
    let again = serde_json::to_value(&parsed).expect("serializes");
    assert_eq!(again, original, "round trip changed {line}");
    parsed
}

fn params<T: DeserializeOwned>(params: Option<Value>) -> T {
    serde_json::from_value(params.expect("has params")).expect("params match their type")
}

#[test]
fn documented_example_session_round_trips() {
    let hello: Request = round_trip(EXAMPLE_HELLO);
    assert_eq!(hello.method, method::HELLO);
    let hello: HelloParams = params(hello.params);
    assert_eq!(hello.proto, PROTO);

    let answer: Response = round_trip(EXAMPLE_HELLO_RESULT);
    let Outcome::Result(result) = answer.outcome else {
        panic!("expected a result")
    };
    let result: HelloResult = params(Some(result));
    assert_eq!(result.seq, 4812);

    let subscribe: Request = round_trip(EXAMPLE_SUBSCRIBE);
    assert_eq!(subscribe.method, method::SUBSCRIBE);
    let topics: TopicsParams = params(subscribe.params);
    assert_eq!(topics.topics, ["hypr.*", "media.player.*"]);

    let event: Notification = round_trip(EXAMPLE_EVENT);
    assert_eq!(event.method, method::EVENT);
    let event: Event = params(event.params);
    assert_eq!(
        (event.v, event.seq, event.ts),
        (EVENT_VERSION, 4813, 1_760_000_000_123)
    );

    // Methods outside proto 1's typed set still travel through the generic envelope.
    let dispatch: Request = round_trip(EXAMPLE_DISPATCH);
    assert_eq!(dispatch.id, Id::Number(3));

    let denied: Response = round_trip(EXAMPLE_UNAUTHORIZED);
    let Outcome::Error(error) = denied.outcome else {
        panic!("expected an error")
    };
    assert_eq!(error.code, ErrorCode::Unauthorized.code());
    assert_eq!(error.message, ErrorCode::Unauthorized.name());
}

#[test]
fn error_codes_match_the_contract() {
    let table: Vec<(i32, &str)> = ErrorCode::ALL
        .iter()
        .map(|c| (c.code(), c.name()))
        .collect();
    assert_eq!(
        table,
        [
            (-32700, "parse_error"),
            (-32600, "invalid_request"),
            (-32601, "method_not_found"),
            (-32602, "invalid_params"),
            (-32001, "version_mismatch"),
            (-32002, "unauthorized"),
            (-32003, "not_ready"),
            (-32004, "rate_limited"),
        ]
    );
    let error = serde_json::to_value(ErrorObject::from(ErrorCode::VersionMismatch)).unwrap();
    assert_eq!(
        error,
        json!({ "code": -32001, "message": "version_mismatch" })
    );
}

#[test]
fn only_json_rpc_2_0_is_accepted() {
    let line = EXAMPLE_HELLO.replace(r#""2.0""#, r#""1.0""#);
    assert!(serde_json::from_str::<Request>(&line).is_err());
}

#[test]
fn response_has_exactly_one_of_result_or_error() {
    let both = r#"{"jsonrpc":"2.0","id":1,"result":{},"error":{"code":-32603,"message":"x"}}"#;
    let neither = r#"{"jsonrpc":"2.0","id":1}"#;
    assert!(serde_json::from_str::<Response>(both).is_err());
    assert!(serde_json::from_str::<Response>(neither).is_err());

    let null_result: Response = round_trip(r#"{"jsonrpc":"2.0","id":1,"result":null}"#);
    assert_eq!(null_result.outcome, Outcome::Result(Value::Null));
}

#[test]
fn response_to_an_unreadable_request_has_a_null_id() {
    let response = Response {
        jsonrpc: JsonRpc::V2,
        id: None,
        outcome: Outcome::Error(ErrorCode::ParseError.into()),
    };
    let line = serde_json::to_value(&response).unwrap();
    assert_eq!(line["id"], Value::Null);
}

#[test]
fn unknown_fields_and_error_codes_are_tolerated() {
    let newer =
        r#"{"jsonrpc":"2.0","id":"a","result":{"proto":1,"daemon":"0.2.0","seq":1,"extra":true}}"#;
    let response: Response = serde_json::from_str(newer).unwrap();
    let Outcome::Result(result) = response.outcome else {
        panic!("expected a result")
    };
    assert!(serde_json::from_value::<HelloResult>(result).is_ok());

    let later_code =
        r#"{"jsonrpc":"2.0","id":1,"error":{"code":-32099,"message":"something_new"}}"#;
    assert!(serde_json::from_str::<Response>(later_code).is_ok());
}

#[test]
fn subscribe_result_is_an_empty_object() {
    assert_eq!(
        serde_json::to_value(EmptyResult::default()).unwrap(),
        json!({})
    );
}

#[test]
fn json_schema_matches_snapshot() {
    let schemas: BTreeMap<&str, schemars::Schema> = BTreeMap::from([
        ("Request", schemars::schema_for!(Request)),
        ("Notification", schemars::schema_for!(Notification)),
        ("Response", schemars::schema_for!(Response)),
        ("HelloParams", schemars::schema_for!(HelloParams)),
        ("HelloResult", schemars::schema_for!(HelloResult)),
        ("TopicsParams", schemars::schema_for!(TopicsParams)),
        ("EmptyResult", schemars::schema_for!(EmptyResult)),
        ("SnapshotResult", schemars::schema_for!(SnapshotResult)),
        ("Event", schemars::schema_for!(Event)),
    ]);
    insta::assert_json_snapshot!("proto-1", schemas);
}
