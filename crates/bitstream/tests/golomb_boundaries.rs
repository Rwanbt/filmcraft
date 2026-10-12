//! Independent range and signed-mapping contracts for Exp-Golomb codes.
use filmcraft_bitstream::{BitError, BitReader, BitWriter};

#[test]
fn truncated_golomb_zero_max_consumes_no_bits() {
    let mut reader = BitReader::new(&[]);
    assert_eq!(reader.read_te(0), Ok(0));
    assert_eq!(reader.position(), 0);
}

#[test]
fn truncated_golomb_rejects_code_values_exceeding_max() {
    let mut writer = BitWriter::new();
    writer.write_ue(3);
    let bits = writer.finish();
    let mut reader = BitReader::new(&bits);
    assert_eq!(reader.read_te(2), Err(BitError::InvalidExpGolomb));
}

#[test]
fn signed_golomb_rejects_positive_values_not_representable_as_i32() {
    let mut writer = BitWriter::new();
    writer.write_ue(u32::MAX);
    let bits = writer.finish();
    let mut reader = BitReader::new(&bits);
    assert_eq!(reader.read_se(), Err(BitError::InvalidExpGolomb));
}

#[test]
fn signed_golomb_preserves_representable_extrema() {
    let mut writer = BitWriter::new();
    for value in [i32::MAX, i32::MIN + 1, 0, -1, 1] {
        writer.write_se(value);
    }
    let bits = writer.finish();
    let mut reader = BitReader::new(&bits);
    for value in [i32::MAX, i32::MIN + 1, 0, -1, 1] {
        assert_eq!(reader.read_se(), Ok(value));
    }
}
