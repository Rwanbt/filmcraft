//! Public bitstream API contract tests with independent byte-level expectations.
//! Covers state transitions, EOF behavior and deterministic corruption handling.

use filmcraft_bitstream::{BitError, BitReader, BitWriter, annexb_nals, crc32, escape_rbsp, length_prefixed_nals, unescape_rbsp};

#[test]
fn reader_cursor_alignment_skip_and_flags_are_consistent() {
    let payload = [0b1010_1100, 0b0101_1000];
    let mut reader = BitReader::new(&payload);
    assert_eq!(reader.data(), &payload);
    assert!(reader.is_byte_aligned());
    assert_eq!(reader.byte_pos(), 0);
    assert_eq!(reader.read_flag(), Ok(true));
    assert_eq!(reader.position(), 1);
    assert!(!reader.is_byte_aligned());
    assert_eq!(reader.skip(6), Ok(()));
    assert_eq!(reader.position(), 7);
    assert_eq!(reader.byte_pos(), 0);
    assert_eq!(reader.read_flag(), Ok(false));
    assert_eq!(reader.position(), 8);
    assert!(reader.is_byte_aligned());
    reader.seek_bits(9);
    assert!(!reader.is_byte_aligned());
    reader.byte_align();
    assert_eq!(reader.position(), 16);
    assert_eq!(reader.skip(1), Err(BitError::Eof));
    assert_eq!(reader.position(), 16);
    assert_eq!(reader.read_flag(), Err(BitError::Eof));
}

#[test]
fn writer_byte_alignment_and_trailing_bits_match_reference_bytes() {
    let mut writer = BitWriter::new();
    assert_eq!(writer.bit_len(), 0);
    assert!(writer.is_byte_aligned());
    assert!(writer.bytes().is_empty());
    writer.write_bytes(&[0xab]);
    assert_eq!(writer.bit_len(), 8);
    assert_eq!(writer.bytes(), &[0xab]);
    writer.write_bits(0b101, 3);
    assert_eq!(writer.bit_len(), 11);
    assert!(!writer.is_byte_aligned());
    writer.write_bytes(&[0x7e]);
    assert_eq!(writer.bit_len(), 19);
    writer.rbsp_trailing();
    assert!(writer.is_byte_aligned());
    assert_eq!(writer.finish(), vec![0xab, 0xaf, 0xd0]);
}

#[test]
fn rbsp_stop_bit_is_not_payload_and_empty_stream_has_no_more_data() {
    for bytes in [&[][..], &[0][..], &[0, 0][..], &[0x80][..]] {
        assert!(!BitReader::new(bytes).more_rbsp_data());
    }
    let mut reader = BitReader::new(&[0b0100_0000]);
    assert!(reader.more_rbsp_data());
    assert_eq!(reader.read_flag(), Ok(false));
    assert!(!reader.more_rbsp_data());
}

#[test]
fn errors_are_human_readable_and_stable() {
    assert_eq!(BitError::Eof.to_string(), "unexpected end of bitstream");
    assert_eq!(BitError::InvalidExpGolomb.to_string(), "invalid Exp-Golomb code");
}

#[test]
fn crc32_matches_independent_ietf_vectors() {
    assert_eq!(crc32(b""), 0);
    assert_eq!(crc32(b"123456789"), 0xcbf4_3926);
    assert_eq!(crc32(b"abc"), 0x3524_41c2);
}

#[test]
fn nal_splitting_handles_zero_length_units_and_invalid_truncations() {
    assert!(annexb_nals(&[]).is_empty());
    assert!(annexb_nals(&[0, 0, 1]).is_empty());
    assert_eq!(annexb_nals(&[0, 0, 1, 0x65, 0, 0, 0, 1, 0x67]), vec![&[0x65][..], &[0x67][..]]);
    // Unlike Annex-B delimiters, length-prefixed containers encode their payload
    // lengths explicitly, including zero-length NALs.
    assert_eq!(length_prefixed_nals(&[0, 1, 0xab, 0, 0], 2), Ok(vec![&[0xab][..], &[][..]]));
    assert_eq!(length_prefixed_nals(&[0, 2, 0xab], 1), Err(BitError::Eof));
    assert_eq!(length_prefixed_nals(&[0], 4), Err(BitError::Eof));
}

#[test]
fn rbsp_escape_is_idempotent_on_payloads_without_escape_sequences() {
    let raw = [0, 0, 4, 0, 0, 5, 0, 0, 0, 1];
    let escaped = escape_rbsp(&raw);
    assert_eq!(escaped, [0, 0, 4, 0, 0, 5, 0, 0, 3, 0, 1]);
    assert_eq!(unescape_rbsp(&escaped), raw);
}

#[test]
fn malformed_exp_golomb_never_silently_returns_a_value() {
    let mut reader = BitReader::new(&[0; 5]);
    assert_eq!(reader.read_ue(), Err(BitError::InvalidExpGolomb));
    let mut reader = BitReader::new(&[0, 0, 0, 0]);
    assert_eq!(reader.read_ue(), Err(BitError::Eof));
}

#[test]
fn deterministic_prefix_mutations_cannot_consume_beyond_source() {
    let mut seed = 0x5e_ed_c0_de_12_34_56_78u64;
    for _ in 0..256 {
        let len = (seed as usize) % 48;
        let mut bytes = Vec::with_capacity(len);
        for _ in 0..len {
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            bytes.push((seed >> 32) as u8);
        }
        for width in [1, 2, 4] {
            if let Ok(nals) = length_prefixed_nals(&bytes, width) {
                let sum: usize = nals.iter().map(|n| n.len()).sum();
                assert!(sum <= bytes.len());
            }
        }
        for nal in annexb_nals(&bytes) {
            assert!(nal.len() <= bytes.len());
        }
    }
}
