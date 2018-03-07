#[macro_use]
extern crate neon;

extern crate nanopow_rs;
use nanopow_rs::{InputHash, Work};

use neon::vm::{Call, JsResult};
use neon::js::{JsString, JsBoolean, JsNumber, JsUndefined, JsFunction};
use neon::js::error::JsError;
use neon::js::error::Kind;
use neon::task::Task;
use neon::scope::Scope;

struct GenerateTask {
    hash: String,
    max_iters: Option<u64>,
}

impl GenerateTask {
    pub fn new(hash: String, max_iters: Option<u64>) -> Self {
        Self {
            hash,
            max_iters,
        }
    }
}

impl Task for GenerateTask {
    type Output = String;
    type Error = String;
    type JsEvent = JsString;

    fn perform(&self) -> Result<Self::Output, Self::Error> {
        let result_work = nanopow_rs::generate_work(&self.hash, self.max_iters);
        match result_work {
            Some(work) => Ok(work),
            None => Err(format!("Did not find valid work."))
        }
    }

    fn complete<'a, T: Scope<'a>>(self, scope: &'a mut T, result: Result<Self::Output, Self::Error>) -> JsResult<Self::JsEvent> {
        match result {
            Ok(w) => Ok(JsString::new(scope, &w).unwrap()),
            Err(e) => JsError::throw(Kind::Error, &e)
        }
    }
}

fn generate_work(call: Call) -> JsResult<JsUndefined> {
    let scope = call.scope;
    let args = call.arguments;
    if args.len() != 3 {
        return JsError::throw(Kind::SyntaxError, "Incorrect number of arguments.")
    };

    let in_hash = args.require(scope, 0)?.check::<JsString>()?.value();
    
    let max_iters = {
        let iters = args.require(scope, 1)?.check::<JsNumber>()?.value() as u64;
        if iters == 0 {
            None
        } else {
            Some(iters)
        }
    };

    let cb = args.require(scope, 2)?.check::<JsFunction>()?;

    let gen_task = GenerateTask::new(in_hash, max_iters);
    gen_task.schedule(cb);
    Ok(JsUndefined::new())
}

fn generate_work_sync(call: Call) -> JsResult<JsString> {
    let scope = call.scope;
    let args = call.arguments;
    if args.len() != 2 {
        return JsError::throw(Kind::SyntaxError, "Incorrect number of arguments.")
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
        return JsError::throw(Kind::SyntaxError, "Incorrect argument length.")
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
    m.export("generateWorkSync", generate_work_sync)?;
    m.export("checkWork", check_work)?;
    Ok(())
});
