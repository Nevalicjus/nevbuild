use std::fs;

#[test]
fn test_c() {
	let _ = fs::File::create("test.c").expect("");
	let s: &[u8] = "#include <stdio.h>\n\nint main() {\n    printf(\"Hello, World!\\n\");\n}\n".as_bytes();
	let _ = fs::write("test.c", &s);
}
