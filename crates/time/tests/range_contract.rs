//! Public half-open range contract regressions for timeline and media editing.
//! Deliberately uses small integer ticks as an independent intersection oracle.

use filmcraft_time::{Tick, TimeRange};

#[test]
fn empty_and_reversed_ranges_never_overlap() {
    let covering = TimeRange::from_bounds(Tick(-50), Tick(50));

    for start in [-100, 0, 100] {
        for duration in [-10, 0] {
            let empty = TimeRange::new(Tick(start), Tick(duration));
            assert!(empty.is_empty());
            assert!(!empty.overlaps(&covering), "empty range {empty:?} overlaps a nonempty range");
            assert!(!covering.overlaps(&empty), "nonempty range overlaps empty range {empty:?}");
            assert_eq!(empty.intersect(&covering), None);
            assert_eq!(covering.intersect(&empty), None);
        }
    }
}

#[test]
fn overlap_matches_half_open_set_intersection() {
    // Exhaustive bounded check, with an oracle independent of overlaps().
    for start_a in -3..=3 {
        for duration_a in -2..=3 {
            for start_b in -3..=3 {
                for duration_b in -2..=3 {
                    let a = TimeRange::new(Tick(start_a), Tick(duration_a));
                    let b = TimeRange::new(Tick(start_b), Tick(duration_b));
                    let expected = duration_a > 0 && duration_b > 0 && start_a < start_b + duration_b && start_b < start_a + duration_a;

                    assert_eq!(a.overlaps(&b), expected, "a={a:?}, b={b:?}");
                    assert_eq!(b.overlaps(&a), expected, "b={b:?}, a={a:?}");
                    assert_eq!(a.intersect(&b).is_some(), expected, "intersection disagrees: a={a:?}, b={b:?}");
                }
            }
        }
    }
}

#[test]
fn touching_nonempty_ranges_do_not_overlap() {
    let before = TimeRange::from_bounds(Tick(2), Tick(5));
    let after = TimeRange::from_bounds(Tick(5), Tick(9));
    assert!(!before.overlaps(&after));
    assert!(!after.overlaps(&before));
    assert_eq!(before.intersect(&after), None);
}
