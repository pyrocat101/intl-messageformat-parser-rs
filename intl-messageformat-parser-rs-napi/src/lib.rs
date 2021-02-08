use intl_messageformat_parser_rs::Parser;

#[macro_use]
extern crate napi;
#[macro_use]
extern crate napi_derive;

// use std::convert::TryInto;

use napi::{CallContext, JsString, Module, Result};

#[cfg(all(unix, not(target_env = "musl")))]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[cfg(windows)]
#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

register_module!(intl_message_format_parser, init);

fn init(module: &mut Module) -> Result<()> {
    module.create_named_method("parse", parse)?;
    Ok(())
}

// TODO: support options
#[js_function(1)]
fn parse(ctx: CallContext) -> Result<JsString> {
    let message = ctx.get::<JsString>(0)?;
    let message = message.as_str()?;

    let mut parser = Parser::new(message, None);
    let ast = parser.parse().map_err(|_| {
        // TODO: give better error message.
        napi::Error::from_reason("Invalid message!".to_string())
    })?;

    let json =
        serde_json::to_string(&ast).map_err(|err| napi::Error::from_reason(err.to_string()))?;

    ctx.env.create_string(&json)
}
