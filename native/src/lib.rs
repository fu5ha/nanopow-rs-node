#[macro_use]
extern crate neon;

extern crate nanopow_rs;

use neon::vm::{Call, JsResult};
use neon::js::{JsString, JsBoolean, Variant};
use neon::js::error::JsError;
use neon::js::error::Kind::SyntaxError;

fn generate_work(call: Call) -> JsResult<JsString> {
    let scope = call.scope;
    let args = call.arguments;
    if args.len() != 1 {
        return Err(JsError::throw(SyntaxError, "Incorrect argument length.").unwrap())
    };
    let in_hash_handle = args.get(scope, 0).unwrap();
    let in_hash_handle = match in_hash_handle.variant() {
        Variant::String(h) => {h},
        _ => {
            return Err(JsError::throw(SyntaxError, "First argument must be a string!").unwrap())
        }
    };
    let in_hash = in_hash_handle.value();

    let result_work = nanopow_rs::generate_work(&in_hash);

    Ok(JsString::new(scope, &result_work).unwrap())
}

fn generate_work_no_limit(call: Call) -> JsResult<JsString> {
    let scope = call.scope;
    let args = call.arguments;
    if args.len() != 1 {
        return Err(JsError::throw(SyntaxError, "Incorrect argument length.").unwrap())
    };
    let in_hash_handle = args.get(scope, 0).unwrap();
    let in_hash_handle = match in_hash_handle.variant() {
        Variant::String(h) => {h},
        _ => {
            return Err(JsError::throw(SyntaxError, "First argument must be a string!").unwrap())
        }
    };
    let in_hash = in_hash_handle.value();

    let result_work = nanopow_rs::generate_work_no_limit(&in_hash);

    Ok(JsString::new(scope, &result_work).unwrap())
}

fn check_work(call: Call) -> JsResult<JsBoolean> {
    let scope = call.scope;
    let args = call.arguments;
    if args.len() != 2 {
        return Err(JsError::throw(SyntaxError, "Incorrect argument length.").unwrap())
    };
    let in_hash_handle = args.get(scope, 0).unwrap();
    let in_hash_handle = match in_hash_handle.variant() {
        Variant::String(h) => {h},
        _ => {
            return Err(JsError::throw(SyntaxError, "First argument must be a string!").unwrap())
        }
    };
    let in_hash = in_hash_handle.value();

    let in_work_handle = args.get(scope, 0).unwrap();
    let in_work_handle = match in_work_handle.variant() {
        Variant::String(h) => {h},
        _ => {
            return Err(JsError::throw(SyntaxError, "Second argument must be a string!").unwrap())
        }
    };
    let in_work = in_work_handle.value();

    let valid = nanopow_rs::check_work(&in_hash, &in_work);

    Ok(JsBoolean::new(scope, valid))
}

register_module!(m, {
    m.export("generateWork", generate_work)?;
    m.export("generateWorkNoLimit", generate_work_no_limit)?;
    m.export("checkWork", check_work)?;
    Ok(())
});
