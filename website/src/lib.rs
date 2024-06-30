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
pub fn process_input(string: &str) -> String {
    let (mut group, positions, bounds, construct_ab_pairs) =
        match crystallography::objects::from_str(string) {
            Ok(args) => args,
            Err(err) => return format!("There was an error during parsing:\n {}", err),
        };
    let expansions = crystallography::objects::calculate_pairs(
        &mut group,
        positions,
        bounds,
        construct_ab_pairs,
    );
    convert_string_for_wasm(crystallography::objects::produce_output_string(&expansions))
}
