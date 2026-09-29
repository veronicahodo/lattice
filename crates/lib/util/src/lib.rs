use anyhow::Result;
use serde_json::Value;

pub fn value_to_string_arr(val: Value) -> Result<Vec<String>> {
    let foo: Vec<String> = val
        .as_array()
        .map(|arr| {
            arr.iter()
                .map(|val| match val {
                    Value::String(s) => s.clone(),
                    _ => val.to_string(), // Converts numbers, bools, etc. to strings
                })
                .collect()
        })
        .unwrap_or_default();
    Ok(foo)
}

pub fn verbose_print(print: String, verbose: bool) {
    if verbose {
        print!("{}", print);
    }
}

pub fn verbose_println(print: String, verbose: bool) {
    if verbose {
        println!("{}", print);
    }
}
