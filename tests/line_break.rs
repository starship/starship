mod common;

#[test]
fn line_break_module() {
    let expected = "\n";
    let actual = common::render_module_with_status("line_break", "~", "0");
    assert_eq!(expected, actual);
}
