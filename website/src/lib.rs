use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

fn convert_string_for_wasm(string: String) -> String {
    string.replace("\n", "<br>")
}

#[wasm_bindgen]
pub fn get_example() -> String {
    include_str!("../../files/input/commented_example").to_string()
}

#[wasm_bindgen]
pub fn the_function(string: &str) -> String {
    convert_string_for_wasm(
        match crystallography::objects::PairCollection::from_str(string) {
            Ok(collection) => collection.produce_output_string(),
            Err(err) => format!("There was an error:\n{}", err),
        },
    )
}
