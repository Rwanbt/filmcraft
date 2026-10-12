//! Bit-reader rejection and cursor-invariance contracts for untrusted codec headers.
use filmcraft_bitstream::{BitError, BitReader};

#[test]
fn reject_bit_counts_outside_the_public_read_width_without_consuming() {
    let payload = [0xff; 10];
    for count in [33u32, 64, u32::MAX] {
        let mut reader = BitReader::new(&payload);
        assert_eq!(reader.read_bits(count), Err(BitError::Eof), "count={count}");
        assert_eq!(reader.position(), 0, "invalid read advanced cursor for count={count}");
        assert_eq!(reader.read_bits(8), Ok(0xff));
    }
    for count in [65u32, u32::MAX] {
        let mut reader = BitReader::new(&payload);
        assert_eq!(reader.read_bits_u64(count), Err(BitError::Eof), "count={count}");
        assert_eq!(reader.position(), 0, "invalid u64 read advanced cursor for count={count}");
    }
}

#[test]
fn truncated_u64_read_is_atomic_and_a_later_valid_read_still_works() {
    let bytes = [0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde];
    let mut reader = BitReader::new(&bytes);
    assert_eq!(reader.read_bits_u64(64), Err(BitError::Eof));
    assert_eq!(reader.position(), 0);
    assert_eq!(reader.read_bits_u64(56), Ok(0x0012_3456_789a_bcde));
    assert_eq!(reader.bits_left(), 0);
}

#[test]
fn alignment_of_out_of_bounds_seek_never_overflows_or_moves_backwards() {
    let mut reader = BitReader::new(&[]);
    reader.seek_bits(usize::MAX);
    reader.byte_align();
    assert_eq!(reader.position(), usize::MAX);
    assert_eq!(reader.read_bits(1), Err(BitError::Eof));
}

#[test]
fn exact_64_bit_reads_and_zero_width_reads_preserve_value_and_cursor() {
    let bytes = [0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef];
    let mut reader = BitReader::new(&bytes);
    assert_eq!(reader.read_bits(0), Ok(0));
    assert_eq!(reader.position(), 0);
    assert_eq!(reader.read_bits_u64(64), Ok(0x0123_4567_89ab_cdef));
    assert_eq!(reader.bits_left(), 0);
    assert_eq!(reader.read_bits_u64(1), Err(BitError::Eof));
}
