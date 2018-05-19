// #![feature(plugin)]
// #![plugin(rocket_codegen)]

// extern crate rocket;

#[macro_use]
extern crate neon;

use neon::js::JsString;
use neon::vm::{Call, JsResult};

fn hello(call: Call) -> JsResult<JsString> {
    let scope = call.scope;
    // rocket::ignite().mount("/", routes![index]).launch();
    Ok(JsString::new(scope, "hello NEON").unwrap())
}

register_module!(m, { m.export("hello", hello) });

// #[get("/")]
// fn index() -> &'static str {
//     "Hello, world!"
// }
