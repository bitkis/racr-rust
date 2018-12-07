
pub(crate) fn string(indent_level: u32) -> String {
    let mut indent = String::new();
    for _ in 0..indent_level {
        indent += "    ";
    }
    indent
}
