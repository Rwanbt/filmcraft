//! Hostile NAL length-prefix parameters and deterministic AVCC/HVCC samples.
use filmcraft_bitstream::{BitError, length_prefixed_nals};

#[test]
fn rejects_unsupported_prefix_widths_even_for_empty_samples() {
    // Zero used to be accepted for an empty sample, and entered a non-advancing
    // loop for a nonempty sample. Invalid widths must always fail immediately.
    for width in [0usize, 3, 5, 8, usize::MAX] {
        assert_eq!(length_prefixed_nals(&[], width), Err(BitError::Eof), "width={width}");
    }
}

#[test]
fn valid_prefix_widths_split_exact_payloads() {
    for width in [1usize, 2, 4] {
        let mut stream = Vec::new();
        for nal in [&[0x65, 0xaa][..], &[0x67][..]] {
            for shift in (0..width).rev() {
                stream.push((nal.len() >> (shift * 8)) as u8);
            }
            stream.extend_from_slice(nal);
        }
        assert_eq!(length_prefixed_nals(&stream, width).unwrap(), vec![&[0x65, 0xaa][..], &[0x67][..]]);
    }
}

#[test]
fn truncated_prefixes_and_payloads_are_rejected() {
    for width in [1usize, 2, 4] {
        if width > 1 {
            assert_eq!(length_prefixed_nals(&vec![0; width - 1], width), Err(BitError::Eof));
        }
        let mut prefix = vec![0; width];
        prefix[width - 1] = 2;
        assert_eq!(length_prefixed_nals(&prefix, width), Err(BitError::Eof));
        prefix.push(0x65);
        assert_eq!(length_prefixed_nals(&prefix, width), Err(BitError::Eof));
    }
}
