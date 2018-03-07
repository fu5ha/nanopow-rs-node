#[macro_use]
extern crate neon;

extern crate nanopow_rs;
use nanopow_rs::{InputHash, Work};

use neon::vm::{Call, JsResult};
use neon::js::{JsString, JsBoolean, JsNumber};
use neon::js::error::JsError;
use neon::js::error::Kind::SyntaxError;

fn generate_work(call: Call) -> JsResult<JsString> {
    let scope = call.scope;
    let args = call.arguments;
    if args.len() != 2 {
        return JsError::throw(SyntaxError, "Incorrect number of arguments.")
    };

    let in_hash = args.require(scope, 0)?.check::<JsString>()?.value();
    if in_hash.len() != 64 {
        return JsError::throw(SyntaxError, "Input hash is the wrong length.")
    }
    let hash = InputHash::from_hex(&in_hash).unwrap();
    
    let max_iters = {
        let iters = args.require(scope, 1)?.check::<JsNumber>()?.value() as u64;
        if iters == 0 {
            None
        } else {
            Some(iters)
        }
    };

    let result_work = nanopow_rs::generate_work(&hash, max_iters);
    let result_str = if let Some(w) = result_work {
        w.into()
    } else {
        String::from("0000000000000000")
    };
    Ok(JsString::new(scope, &result_str).unwrap())
}

fn check_work(call: Call) -> JsResult<JsBoolean> {
    let scope = call.scope;
    let args = call.arguments;
    if args.len() != 2 {
        return JsError::throw(SyntaxError, "Incorrect argument length.")
    };

    let in_hash = args.require(scope, 0)?.check::<JsString>()?.value();
    if in_hash.len() != 64 {
        return JsError::throw(SyntaxError, "Input hash is the wrong length.")
    }
    let in_work = args.require(scope, 1)?.check::<JsString>()?.value();
    if in_work.len() != 16 {
        return JsError::throw(SyntaxError, "Input work is the wrong length.")
    }
    let hash = InputHash::from_hex(&in_hash).unwrap();
    let work = Work::from_hex(&in_work).unwrap();

    let valid = nanopow_rs::check_work(&hash, &work);
    Ok(JsBoolean::new(scope, valid))
}

register_module!(m, {
    m.export("generateWork", generate_work)?;
    m.export("checkWork", check_work)?;
    Ok(())
});
