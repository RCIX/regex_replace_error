
extern crate regex;
use regex::Regex;

fn main() {
    let source : String = String::from_str("itado");
    let replace : String = String::from_str("t$1t");
    let regex : Regex = match Regex::new(&"t(.)d") {
        Ok(res) => res,
        Err(err) => panic!("Error '{}' with regex '{}'", err, &"t(.)d")
    };
    let value = match regex.is_match(&source) {
        true => regex.replace_all(&source, &replace),
        false => String::from_str("none"),
    };

}
