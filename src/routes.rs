use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use deno_core::op;
use deno_core::Extension;
use deno_core::JsRuntime;
use deno_core::Op;
use deno_core::RuntimeOptions;
use serde::Deserialize;

/// An op for summing an array of numbers. The op-layer automatically
/// deserializes inputs and serializes the returned Result & value.
#[op]
fn op_sum(nums: Vec<f64>) -> Result<f64, deno_core::error::AnyError> {
    // Sum inputs
    let sum = nums.iter().fold(0.0, |a, v| a + v);
    // return as a Result<f64, AnyError>
    Ok(sum)
}

pub async fn index() -> impl IntoResponse {
    // Build a deno_core::Extension providing custom ops
    let ext = Extension {
        name: "my_ext",
        ops: std::borrow::Cow::Borrowed(&[op_sum::DECL]),
        ..Default::default()
    };

    // Initialize a runtime instance
    let mut runtime = JsRuntime::new(RuntimeOptions {
        extensions: vec![ext],
        ..Default::default()
    });

    let script = r#"
    ((globalThis) => {
      const originalConsole = console;
      const consoleProxy = new Proxy(originalConsole, {
        get(_target, propKey) {
          return function (...args) {
            Deno?.core?.print(JSON.stringify({ function: propKey, args })+"\n");
          };
        },
      });
      globalThis.console = consoleProxy;
    })(globalThis);
    
    console.log("Hello World!");
    
    const arr = [1, 2, 3];
    console.log("The sum of");
    console.log(arr);
    console.log("is");
    console.log(Deno.core.ops.op_sum(arr));
    
    // And incorrect usage
    try {
      console.log(Deno.core.ops.op_sum(0));
    } catch(e) {
      console.log('Exception:');
      console.log(e);
    }
    "#;

    // Now we see how to invoke the op we just defined. The runtime automatically
    // contains a Deno.core object with several functions for interacting with it.
    // You can find its definition in core.js.
    let runtime_result = runtime.execute_script_static("<usage>", script);

    return match runtime_result {
        Ok(_) => (StatusCode::OK, "Yaay!".to_string()),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
    };
}
