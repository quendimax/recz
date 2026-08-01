use recz_adt::Legible;

#[test]
fn u8_display() {
    assert_eq!(0.legible().to_string(), r"'\x00'");
    assert_eq!(b'\t'.legible().to_string(), r"'\x09'");
    assert_eq!(b'\r'.legible().to_string(), r"'\x0D'");
    assert_eq!(b'\n'.legible().to_string(), r"'\x0A'");
    assert_eq!(b'\''.legible().to_string(), r"'\''");
    assert_eq!(b'"'.legible().to_string(), r#"'"'"#);
    assert_eq!(b'\\'.legible().to_string(), r"'\\'");
    assert_eq!(0x1B.legible().to_string(), r"'\x1B'");
    assert_eq!(0x1f.legible().to_string(), r"'\x1F'");
    assert_eq!(b' '.legible().to_string(), "' '");
    assert_eq!(b'a'.legible().to_string(), "'a'");
    assert_eq!(0x7F.legible().to_string(), r"'\x7F'");
    assert_eq!(129.legible().to_string(), r"'\x81'");
    assert_eq!(255.legible().to_string(), r"'\xFF'");
}

#[test]
fn u8_arr_display() {
    assert_eq!(
        [0, 1, 2, b'\'', b'"', b'\\'].legible().to_string(),
        r#""\x00\x01\x02'\"\\""#
    );
}
