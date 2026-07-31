use pretty_assertions::{assert_eq, assert_ne};
use recz_adt::{RangeU8, SetU8};

// ── Construction ──────────────────────────────────────────────────────────────

#[test]
fn set_u8_new_is_empty() {
    let set = SetU8::new();
    assert_eq!(set.len(), 0);
    assert!(set.is_empty());
    assert_eq!(set.capacity(), 256);
}

#[test]
fn set_u8_default_is_empty() {
    let set = SetU8::default();
    assert!(set.is_empty());
    assert_eq!(set.capacity(), 256);
}

#[test]
fn set_u8_full() {
    let set = SetU8::full();
    assert_eq!(set.len(), set.capacity());
    assert!(!set.is_empty());
}

// ── From conversions ──────────────────────────────────────────────────────────

#[test]
fn set_u8_from_u8() {
    let set = SetU8::from(b'a');
    assert_eq!(set.len(), 1);
    assert!(set.contains(b'a'));
    assert!(!set.contains(b'b'));
}

#[test]
fn set_u8_from_range_inclusive() {
    let set = SetU8::from(b'a'..=b'e');
    assert_eq!(set.len(), 5);
    for b in b'a'..=b'e' {
        assert!(set.contains(b));
    }
    assert!(!set.contains(b'f'));
}

#[test]
fn set_u8_from_range_u8() {
    let r = RangeU8::new(1, 5);
    let set = SetU8::from(r);
    assert_eq!(set.len(), 5);
    for b in 1u8..=5 {
        assert!(set.contains(b));
    }
    assert!(!set.contains(0));
    assert!(!set.contains(6));
}

#[test]
fn set_u8_from_range_u8_ref() {
    let r = RangeU8::new(10, 20);
    let set = SetU8::from(&r);
    assert_eq!(set.len(), 11);
    for b in 10u8..=20 {
        assert!(set.contains(b));
    }
}

#[test]
fn set_u8_from_slice() {
    let bytes: &[u8] = &[1, 3, 5];
    let set = SetU8::from(bytes);
    assert_eq!(set.len(), 3);
    assert!(set.contains(1));
    assert!(set.contains(3));
    assert!(set.contains(5));
    assert!(!set.contains(2));
}

#[test]
fn set_u8_from_array() {
    let set = SetU8::from([2u8, 4, 6, 8]);
    assert_eq!(set.len(), 4);
    for b in [2u8, 4, 6, 8] {
        assert!(set.contains(b));
    }
    assert!(!set.contains(1));
}

#[test]
fn set_u8_from_array_ref() {
    let set = SetU8::from(&[10u8, 20, 30]);
    assert_eq!(set.len(), 3);
    assert!(set.contains(10));
    assert!(set.contains(20));
    assert!(set.contains(30));
}

#[test]
fn set_u8_from_deduplicates() {
    let set = SetU8::from([7u8, 7, 7]);
    assert_eq!(set.len(), 1);
    assert!(set.contains(7));
}

#[test]
fn set_u8_from_set_ref() {
    let a = SetU8::from([1u8, 2, 3]);
    let b = SetU8::from(&a);
    assert_eq!(a, b);
}

// ── len / is_empty ────────────────────────────────────────────────────────────

#[test]
fn set_u8_len_tracks_mutations() {
    let set = SetU8::new();
    assert_eq!(set.len(), 0);
    set.insert(10);
    assert_eq!(set.len(), 1);
    set.insert(20);
    assert_eq!(set.len(), 2);
    set.insert(10); // duplicate
    assert_eq!(set.len(), 2);
    set.remove(10);
    assert_eq!(set.len(), 1);
}

#[test]
fn set_u8_is_empty_toggles() {
    let set = SetU8::new();
    assert!(set.is_empty());
    set.insert(0);
    assert!(!set.is_empty());
    set.remove(0);
    assert!(set.is_empty());
}

// ── clear ─────────────────────────────────────────────────────────────────────

#[test]
fn set_u8_clear() {
    let set = SetU8::from([1u8, 2, 3]);
    assert!(!set.is_empty());
    set.clear();
    assert!(set.is_empty());
    assert_eq!(set.len(), 0);
}

// ── insert ────────────────────────────────────────────────────────────────────

#[test]
fn set_u8_insert_returns_newly_inserted() {
    let set = SetU8::new();
    assert_eq!(set.insert(42), true);
    assert_eq!(set.insert(42), false);
    assert_eq!(set.len(), 1);
}

#[test]
fn set_u8_insert_byte() {
    let set = SetU8::new();
    set.insert_byte(7);
    set.insert_byte(7); // idempotent
    assert_eq!(set.len(), 1);
    assert!(set.contains(7));
}

#[test]
fn set_u8_insert_bytes_from_array() {
    let set = SetU8::new();
    set.insert_bytes([10u8, 20, 30]);
    assert_eq!(set.len(), 3);
    assert!(set.contains(10));
    assert!(set.contains(20));
    assert!(set.contains(30));
}

#[test]
fn set_u8_insert_bytes_from_range_u8() {
    let set = SetU8::new();
    set.insert_bytes(RangeU8::new(1, 4));
    assert_eq!(set.len(), 4);
    for b in 1u8..=4 {
        assert!(set.contains(b));
    }
}

#[test]
fn set_u8_insert_bytes_from_range_inclusive() {
    let set = SetU8::new();
    set.insert_bytes(b'A'..=b'Z');
    assert_eq!(set.len(), 26);
    assert!(set.contains(b'A'));
    assert!(set.contains(b'Z'));
    assert!(!set.contains(b'a'));
}

#[test]
fn set_u8_insert_bytes_from_set() {
    let extra = SetU8::from([5u8, 6, 7]);
    let set = SetU8::from([1u8, 2]);
    set.insert_bytes(extra);
    assert_eq!(set.len(), 5);
}

#[test]
fn set_u8_insert_bytes_overlap() {
    let set = SetU8::from([1u8, 2]);
    set.insert_bytes([2u8, 3]);
    assert_eq!(set.len(), 3);
    assert!(set.contains(1));
    assert!(set.contains(2));
    assert!(set.contains(3));
}

// ── remove ────────────────────────────────────────────────────────────────────

#[test]
fn set_u8_remove_present() {
    let set = SetU8::from([1u8, 2, 3]);
    assert_eq!(set.remove(2), true);
    assert_eq!(set.remove(2), false);
    assert_eq!(set.len(), 2);
    assert!(!set.contains(2));
}

#[test]
fn set_u8_remove_absent() {
    let set = SetU8::from([1u8, 3]);
    assert_eq!(set.remove(2), false);
    assert_eq!(set.len(), 2);
}

#[test]
fn set_u8_remove_bytes() {
    let set = SetU8::from([1u8, 2, 3, 4]);
    set.remove_bytes([2u8, 3]);
    let v: Vec<u8> = set.iter().collect();
    assert_eq!(v, [1, 4]);
}

#[test]
fn set_u8_remove_bytes_from_range() {
    let set = SetU8::from(0u8..=10u8);
    set.remove_bytes(RangeU8::new(3, 7));
    assert_eq!(set.len(), 6);
    let v: Vec<u8> = set.iter().collect();
    assert_eq!(v, [0, 1, 2, 8, 9, 10]);
}

// ── take ──────────────────────────────────────────────────────────────────────

#[test]
fn set_u8_take_present_then_absent() {
    let set = SetU8::from([1u8, 2, 3]);
    assert_eq!(set.take(&2), Some(2));
    assert_eq!(set.take(&2), None);
    assert_eq!(set.len(), 2);
    assert!(!set.contains(2));
}

#[test]
fn set_u8_take_absent() {
    let set = SetU8::from([1u8, 3]);
    assert_eq!(set.take(&5), None);
    assert_eq!(set.len(), 2);
}

// ── first / last ──────────────────────────────────────────────────────────────

#[test]
fn set_u8_first_last_empty() {
    let set = SetU8::new();
    assert_eq!(set.first(), None);
    assert_eq!(set.last(), None);
}

#[test]
fn set_u8_first_last_single() {
    let set = SetU8::from([42u8]);
    assert_eq!(set.first(), Some(42));
    assert_eq!(set.last(), Some(42));
}

#[test]
fn set_u8_first_last_multiple() {
    let set = SetU8::from([50u8, 10, 200]);
    assert_eq!(set.first(), Some(10));
    assert_eq!(set.last(), Some(200));
}

#[test]
fn set_u8_first_is_byte_zero() {
    let set = SetU8::from([0u8, 100, 200]);
    assert_eq!(set.first(), Some(0));
}

#[test]
fn set_u8_last_is_byte_255() {
    let set = SetU8::from([1u8, 100, 255]);
    assert_eq!(set.last(), Some(255));
}

#[test]
fn set_u8_first_last_boundary_bytes() {
    let set = SetU8::from([0u8, 255]);
    assert_eq!(set.first(), Some(0));
    assert_eq!(set.last(), Some(255));
}

// ── contains / contains_bytes ─────────────────────────────────────────────────

#[test]
fn set_u8_contains() {
    let set = SetU8::from([1u8, 2, 3]);
    assert!(set.contains(1));
    assert!(set.contains(2));
    assert!(set.contains(3));
    assert!(!set.contains(0));
    assert!(!set.contains(4));
}

#[test]
fn set_u8_contains_boundary_bytes() {
    let set = SetU8::from([0u8, 255]);
    assert!(set.contains(0));
    assert!(set.contains(255));
    assert!(!set.contains(1));
    assert!(!set.contains(254));
}

#[test]
fn set_u8_contains_bytes_subset() {
    let set = SetU8::from([1u8, 2, 3, 4]);
    assert!(set.contains_bytes([1u8, 2]));
    assert!(set.contains_bytes([1u8, 2, 3, 4]));
}

#[test]
fn set_u8_contains_bytes_not_subset() {
    let set = SetU8::from([1u8, 2, 3]);
    assert!(!set.contains_bytes([1u8, 2, 5]));
    assert!(!set.contains_bytes([5u8]));
}

#[test]
fn set_u8_contains_bytes_from_range() {
    let set = SetU8::from(0u8..=20u8);
    assert!(set.contains_bytes(RangeU8::new(5, 15)));
    assert!(!set.contains_bytes(RangeU8::new(10, 25)));
}

// ── set algebra ───────────────────────────────────────────────────────────────

#[test]
fn set_u8_difference() {
    let a = SetU8::from([1u8, 2, 3]);
    let b = SetU8::from([2u8, 3, 4]);
    let diff: Vec<u8> = a.difference(&b).iter().collect();
    assert_eq!(diff, [1]);
}

#[test]
fn set_u8_difference_is_not_symmetric() {
    let a = SetU8::from([1u8, 2, 3]);
    let b = SetU8::from([2u8, 3, 4]);
    let a_minus_b: Vec<u8> = a.difference(&b).iter().collect();
    let b_minus_a: Vec<u8> = b.difference(&a).iter().collect();
    assert_eq!(a_minus_b, [1]);
    assert_eq!(b_minus_a, [4]);
}

#[test]
fn set_u8_difference_with_disjoint() {
    let a = SetU8::from([1u8, 2]);
    let b = SetU8::from([3u8, 4]);
    let diff: Vec<u8> = a.difference(&b).iter().collect();
    assert_eq!(diff, [1, 2]); // all of a is preserved
}

#[test]
fn set_u8_symmetric_difference() {
    let a = SetU8::from([1u8, 2, 3]);
    let b = SetU8::from([2u8, 3, 4]);
    let sym_diff: Vec<u8> = a.symmetric_difference(&b).iter().collect();
    assert_eq!(sym_diff, [1, 4]);
}

#[test]
fn set_u8_symmetric_difference_is_symmetric() {
    let a = SetU8::from([1u8, 2, 3]);
    let b = SetU8::from([2u8, 3, 4]);
    assert_eq!(a.symmetric_difference(&b), b.symmetric_difference(&a));
}

#[test]
fn set_u8_intersection() {
    let a = SetU8::from([1u8, 2, 3]);
    let b = SetU8::from([2u8, 3, 4]);
    let inter: Vec<u8> = a.intersection(&b).iter().collect();
    assert_eq!(inter, [2, 3]);
}

#[test]
fn set_u8_intersection_disjoint_is_empty() {
    let a = SetU8::from([1u8, 2]);
    let b = SetU8::from([3u8, 4]);
    assert!(a.intersection(&b).is_empty());
}

#[test]
fn set_u8_union() {
    let a = SetU8::from([1u8, 2]);
    let b = SetU8::from([2u8, 3]);
    let u: Vec<u8> = a.union(&b).iter().collect();
    assert_eq!(u, [1, 2, 3]);
}

#[test]
fn set_u8_union_with_empty() {
    let a = SetU8::from([1u8, 2, 3]);
    let b = SetU8::new();
    assert_eq!(a.union(&b), a);
}

// ── iterators ─────────────────────────────────────────────────────────────────

#[test]
fn set_u8_iter_empty() {
    let set = SetU8::new();
    let v: Vec<u8> = set.iter().collect();
    assert_eq!(v, [] as [u8; 0]);
}

#[test]
fn set_u8_iter_into_set() {
    let set = SetU8::from([1, 5, 89]);
    let iter = set.iter();
    let set2 = iter.into_set();
    assert_eq!(set, set2);
}

#[test]
fn set_u8_iter_yields_sorted() {
    let set = SetU8::from([50u8, 10, 200, 1]);
    let v: Vec<u8> = set.iter().collect();
    assert_eq!(v, [1, 10, 50, 200]);
}

#[test]
fn set_u8_bytes_is_alias_for_iter() {
    let set = SetU8::from([3u8, 1, 4, 1, 5]); // duplicate 1 is ignored
    let via_iter: Vec<u8> = set.iter().collect();
    let via_bytes: Vec<u8> = set.bytes().collect();
    assert_eq!(via_iter, [1, 3, 4, 5]);
    assert_eq!(via_iter, via_bytes);
}

#[test]
fn set_u8_iter_includes_boundary_bytes() {
    let set = SetU8::from([0u8, 127, 128, 255]);
    let v: Vec<u8> = set.iter().collect();
    assert_eq!(v, [0, 127, 128, 255]);
}

#[test]
fn set_u8_iter_full_256_bytes() {
    let set = SetU8::from(0u8..=255u8);
    assert_eq!(set.len(), 256);
    let v: Vec<u8> = set.iter().collect();
    assert_eq!(v.len(), 256);
    assert_eq!(v[0], 0);
    assert_eq!(v[255], 255);
    assert!(v.windows(2).all(|w| w[0] < w[1]));
}

#[test]
fn set_u8_ranges_empty() {
    let set = SetU8::new();
    let v: Vec<RangeU8> = set.ranges().collect();
    assert_eq!(v, [] as [RangeU8; 0]);
}

#[test]
fn set_u8_ranges_into_set() {
    let set = SetU8::from([1, 5, 89]);
    let iter = set.ranges();
    let set2 = iter.into_set();
    assert_eq!(set, set2);
}

#[test]
fn set_u8_ranges_single_byte() {
    let set = SetU8::from([42u8]);
    let v: Vec<RangeU8> = set.ranges().collect();
    assert_eq!(v, [RangeU8::new(42, 42)]);
}

#[test]
fn set_u8_ranges_contiguous_run() {
    let set = SetU8::from(b'a'..=b'e');
    let v: Vec<RangeU8> = set.ranges().collect();
    assert_eq!(v, [RangeU8::new(b'a', b'e')]);
}

#[test]
fn set_u8_ranges_disjoint_bytes() {
    let set = SetU8::from([1u8, 5, 10]);
    let v: Vec<RangeU8> = set.ranges().collect();
    assert_eq!(
        v,
        [RangeU8::new(1, 1), RangeU8::new(5, 5), RangeU8::new(10, 10)]
    );
}

#[test]
fn set_u8_ranges_across_chunk_boundary() {
    // Bytes 62-65 straddle the boundary between chunk 0 (bytes 0-63) and
    // chunk 1 (bytes 64-127). The per-chunk iterator yields two separate ranges.
    let set = SetU8::from(62u8..=65u8);
    let v: Vec<RangeU8> = set.ranges().collect();
    assert_eq!(v, [RangeU8::new(62, 65)]);
    assert_eq!(set.len(), 4);
}

#[test]
fn set_u8_ranges_all_chunk_boundaries() {
    // One byte from each side of every chunk boundary: 63, 64, 127, 128, 191, 192.
    let set = SetU8::from([63u8, 64, 127, 128, 191, 192]);
    let v: Vec<RangeU8> = set.ranges().collect();
    assert_eq!(
        v,
        [
            RangeU8::new(63, 64),
            RangeU8::new(127, 128),
            RangeU8::new(191, 192),
        ]
    );
}

// ── bitwise operators ─────────────────────────────────────────────────────────

#[test]
fn set_u8_bitor() {
    let a = SetU8::from([1u8, 2]);
    let b = SetU8::from([2u8, 3]);
    let v: Vec<u8> = (a | b).iter().collect();
    assert_eq!(v, [1, 2, 3]);
}

#[test]
fn set_u8_bitand() {
    let a = SetU8::from([1u8, 2, 3]);
    let b = SetU8::from([2u8, 3, 4]);
    let v: Vec<u8> = (a & b).iter().collect();
    assert_eq!(v, [2, 3]);
}

#[test]
fn set_u8_bitxor() {
    let a = SetU8::from([1u8, 2, 3]);
    let b = SetU8::from([2u8, 3, 4]);
    let v: Vec<u8> = (a ^ b).iter().collect();
    assert_eq!(v, [1, 4]);
}

#[test]
fn set_u8_not_gives_complement() {
    let set = SetU8::from(1u8..=254u8);
    let complement = !set;
    assert_eq!(complement.len(), 2);
    assert!(complement.contains(0));
    assert!(complement.contains(255));
}

#[test]
fn set_u8_not_of_empty_is_full() {
    let set = SetU8::new();
    let all = !set;
    assert_eq!(all.len(), 256);
}

#[test]
fn set_u8_not_of_full_is_empty() {
    let set = SetU8::from(0u8..=255u8);
    let none = !set;
    assert!(none.is_empty());
}

#[test]
fn set_u8_bitor_assign() {
    let mut a = SetU8::from([1u8, 2]);
    a |= SetU8::from([2u8, 3]);
    let v: Vec<u8> = a.iter().collect();
    assert_eq!(v, [1, 2, 3]);
}

#[test]
fn set_u8_bitand_assign() {
    let mut a = SetU8::from([1u8, 2, 3]);
    a &= SetU8::from([2u8, 3, 4]);
    let v: Vec<u8> = a.iter().collect();
    assert_eq!(v, [2, 3]);
}

#[test]
fn set_u8_bitxor_assign() {
    let mut a = SetU8::from([1u8, 2, 3]);
    a ^= SetU8::from([2u8, 3, 4]);
    let v: Vec<u8> = a.iter().collect();
    assert_eq!(v, [1, 4]);
}

#[test]
fn set_u8_ops_with_u8_rhs() {
    let a = SetU8::from([1u8, 2, 3]);
    let v: Vec<u8> = (a | 4u8).iter().collect();
    assert_eq!(v, [1, 2, 3, 4]);

    let a = SetU8::from([1u8, 2, 3]);
    let v: Vec<u8> = (a & 2u8).iter().collect();
    assert_eq!(v, [2]);

    let a = SetU8::from([1u8, 2, 3]);
    let v: Vec<u8> = (a ^ 2u8).iter().collect();
    assert_eq!(v, [1, 3]);
}

#[test]
fn set_u8_ops_with_range_u8_rhs() {
    let a = SetU8::from([1u8, 2, 3, 10]);
    let r = RangeU8::new(2, 5);
    let v: Vec<u8> = (a & r).iter().collect();
    assert_eq!(v, [2, 3]);

    let a = SetU8::from([1u8, 10]);
    let v: Vec<u8> = (a | RangeU8::new(2, 5)).iter().collect();
    assert_eq!(v, [1, 2, 3, 4, 5, 10]);
}

#[test]
fn set_u8_ops_with_range_u8_ref_rhs() {
    let a = SetU8::from([1u8, 2, 3]);
    let r = RangeU8::new(2, 4);
    let v: Vec<u8> = (a & r).iter().collect();
    assert_eq!(v, [2, 3]);
}

#[test]
fn set_u8_ops_with_range_inclusive_rhs() {
    let a = SetU8::from([1u8, 5, 10]);
    let v: Vec<u8> = (a | (3u8..=6u8)).iter().collect();
    assert_eq!(v, [1, 3, 4, 5, 6, 10]);
}

#[test]
fn set_u8_ops_with_set_ref_rhs() {
    let a = SetU8::from([1u8, 2, 3]);
    let b = SetU8::from([2u8, 3, 4]);
    let v: Vec<u8> = (a | b).iter().collect();
    assert_eq!(v, [1, 2, 3, 4]);
}

// ── Display ───────────────────────────────────────────────────────────────────

#[test]
fn set_u8_display_empty() {
    let set = SetU8::new();
    assert_eq!(format!("{set}"), "[]");
}

#[test]
fn set_u8_display_single_printable_byte() {
    let set = SetU8::from(b'a');
    assert_eq!(format!("{set}"), "['a']");
}

#[test]
fn set_u8_display_single_nonprintable_byte() {
    let set = SetU8::from(0u8);
    assert_eq!(format!("{set}"), "[00h]");
}

#[test]
fn set_u8_display_byte_255() {
    let set = SetU8::from(255u8);
    assert_eq!(format!("{set}"), "[FFh]");
}

#[test]
fn set_u8_display_contiguous_printable_range() {
    let set = SetU8::from(b'a'..=b'e');
    assert_eq!(format!("{set}"), "['a'-'e']");
}

#[test]
fn set_u8_display_nonprintable_range() {
    let set = SetU8::from(1u8..=3u8);
    assert_eq!(format!("{set}"), "[01h-03h]");
}

#[test]
fn set_u8_display_mixed_printable_nonprintable_range() {
    // 0x00 to 'Z' (0x5A): starts non-printable, ends printable
    let set = SetU8::from(0u8..=b'Z');
    assert_eq!(format!("{set}"), "[00h-'Z']");
}

#[test]
fn set_u8_display_disjoint_ranges() {
    let set = SetU8::from(b"az");
    assert_eq!(format!("{set}"), "['a' | 'z']");
}

#[test]
fn set_u8_display_three_disjoint_ranges() {
    let a = SetU8::from([1u8, 5, 200]);
    assert_eq!(format!("{a}"), "[01h | 05h | C8h]");
}

#[test]
fn set_u8_display_merges_adjacent_ranges_across_chunk_boundary() {
    // Bytes 62–65 straddle the chunk-0/chunk-1 boundary.
    // ranges() yields [62,63] and [64,65], but Display merges them because
    // 63.steps_between(64) == 1.
    // 62 = '>', 65 = 'A'
    let set = SetU8::from(62u8..=65u8);
    assert_eq!(format!("{set}"), "['>'-'A']");
}

#[test]
fn set_u8_display_does_not_merge_non_adjacent_across_boundary() {
    // Bytes 62, 63, 65, 66 — gap at 64 prevents merging.
    // 62='>', 63='?', 65='A', 66='B'
    let mut set = SetU8::from([62u8, 63, 65, 66]);
    assert_eq!(format!("{set}"), "['>'-'?' | 'A'-'B']");

    // Similarly across chunk boundary at 128: bytes 127 and 129 (gap at 128).
    set = SetU8::from([127u8, 129]);
    assert_eq!(format!("{set}"), "[7Fh | 81h]");
}

#[test]
fn set_u8_display_special_char_single_quote() {
    // b'\'' (ASCII 39) is escaped to '\'' in legible display.
    let set = SetU8::from(b'\'');
    assert_eq!(format!("{set}"), r"['\'']");
}

#[test]
fn set_u8_display_special_char_backslash() {
    // b'\\' (ASCII 92) is escaped to '\\' in legible display.
    let set = SetU8::from(b'\\');
    assert_eq!(format!("{set}"), r"['\\']");
}

// ── Clone / PartialEq / Ord ───────────────────────────────────────────────────

#[test]
fn set_u8_clone_equals_original() {
    let a = SetU8::from([1u8, 2, 3]);
    let b = a.clone();
    assert_eq!(a, b);
}

#[test]
fn set_u8_ne_after_mutation() {
    let a = SetU8::from([1u8, 2, 3]);
    let b = a.clone();
    b.insert(4);
    assert_ne!(a, b);
}

#[test]
fn set_u8_ord_smaller_first_differing_byte() {
    // {1,2} < {1,3} because 2 < 3 at first differing position
    let a = SetU8::from([1u8, 2]);
    let b = SetU8::from([1u8, 3]);
    assert!(a < b);
}

// ── full ──────────────────────────────────────────────────────────────────────

#[test]
fn set_u8_full_has_all_256_bytes() {
    let set = SetU8::full();
    assert_eq!(set.len(), 256);
    assert_eq!(set.capacity(), 256);
    assert!(!set.is_empty());
}

#[test]
fn set_u8_full_contains_boundary_bytes() {
    let set = SetU8::full();
    assert!(set.contains(0));
    assert!(set.contains(127));
    assert!(set.contains(128));
    assert!(set.contains(255));
}

#[test]
fn set_u8_full_is_complement_of_new() {
    // !new() should equal full() and vice versa
    assert_eq!(!SetU8::new(), SetU8::full());
    assert_eq!(!SetU8::full(), SetU8::new());
}

#[test]
fn set_u8_full_iter_covers_all_bytes() {
    let v: Vec<u8> = SetU8::full().iter().collect();
    assert_eq!(v.len(), 256);
    assert_eq!(v[0], 0);
    assert_eq!(v[255], 255);
}

#[test]
fn set_u8_full_ranges_is_single_span() {
    let v: Vec<RangeU8> = SetU8::full().ranges().collect();
    assert_eq!(v[0], RangeU8::new(0, 255));
}

#[test]
fn set_u8_full_display() {
    // All bytes are adjacent, so Display collapses everything to one range.
    // 0 = 00h, 255 = FFh.
    assert_eq!(format!("{}", SetU8::full()), "[00h-FFh]");
}

// ── is_disjoint ───────────────────────────────────────────────────────────────

#[test]
fn set_u8_is_disjoint_with_empty() {
    let a = SetU8::from([1u8, 2, 3]);
    assert!(a.is_disjoint(&SetU8::new()));
    assert!(SetU8::new().is_disjoint(&a));
}

#[test]
fn set_u8_is_disjoint_empty_with_empty() {
    assert!(SetU8::new().is_disjoint(&SetU8::new()));
}

#[test]
fn set_u8_is_disjoint_no_overlap() {
    let a = SetU8::from([1u8, 2, 3]);
    let b = SetU8::from([4u8, 5, 6]);
    assert!(a.is_disjoint(&b));
    assert!(b.is_disjoint(&a)); // symmetric
}

#[test]
fn set_u8_is_disjoint_with_overlap() {
    let a = SetU8::from([1u8, 2, 3]);
    let b = SetU8::from([3u8, 4, 5]);
    assert!(!a.is_disjoint(&b));
    assert!(!b.is_disjoint(&a)); // symmetric
}

#[test]
fn set_u8_is_disjoint_with_self() {
    // A non-empty set is never disjoint with itself.
    let a = SetU8::from([42u8]);
    assert!(!a.is_disjoint(&a));
    // An empty set is disjoint with itself.
    assert!(SetU8::new().is_disjoint(&SetU8::new()));
}

#[test]
fn set_u8_is_disjoint_boundary_bytes() {
    let a = SetU8::from([0u8, 255]);
    let b = SetU8::from([1u8, 254]);
    assert!(a.is_disjoint(&b));
    let c = SetU8::from([0u8]);
    assert!(!a.is_disjoint(&c));
}

// ── is_subset ─────────────────────────────────────────────────────────────────

#[test]
fn set_u8_is_subset_empty_is_subset_of_everything() {
    let empty = SetU8::new();
    assert!(empty.is_subset(&SetU8::new()));
    assert!(empty.is_subset(&SetU8::from([1u8, 2])));
    assert!(empty.is_subset(&SetU8::full()));
}

#[test]
fn set_u8_is_subset_of_itself() {
    let a = SetU8::from([1u8, 2, 3]);
    assert!(a.is_subset(&a));
}

#[test]
fn set_u8_is_subset_proper_subset() {
    let a = SetU8::from([1u8, 2]);
    let b = SetU8::from([1u8, 2, 3]);
    assert!(a.is_subset(&b));
    assert!(!b.is_subset(&a)); // b is a strict superset of a
}

#[test]
fn set_u8_is_subset_partial_overlap_is_not_subset() {
    let a = SetU8::from([1u8, 2, 3]);
    let b = SetU8::from([2u8, 3, 4]);
    assert!(!a.is_subset(&b)); // 1 is in a but not b
    assert!(!b.is_subset(&a)); // 4 is in b but not a
}

#[test]
fn set_u8_is_subset_of_full() {
    // Every set is a subset of the full set.
    assert!(SetU8::new().is_subset(&SetU8::full()));
    assert!(SetU8::from([0u8, 128, 255]).is_subset(&SetU8::full()));
    assert!(SetU8::full().is_subset(&SetU8::full()));
}

#[test]
fn set_u8_is_subset_full_is_not_subset_of_non_full() {
    assert!(!SetU8::full().is_subset(&SetU8::from([1u8, 2])));
}

// ── is_superset ───────────────────────────────────────────────────────────────

#[test]
fn set_u8_is_superset_of_empty() {
    // Every set is a superset of the empty set.
    assert!(SetU8::new().is_superset(&SetU8::new()));
    assert!(SetU8::from([1u8, 2]).is_superset(&SetU8::new()));
    assert!(SetU8::full().is_superset(&SetU8::new()));
}

#[test]
fn set_u8_is_superset_of_itself() {
    let a = SetU8::from([1u8, 2, 3]);
    assert!(a.is_superset(&a));
}

#[test]
fn set_u8_is_superset_proper_superset() {
    let a = SetU8::from([1u8, 2, 3]);
    let b = SetU8::from([1u8, 2]);
    assert!(a.is_superset(&b));
    assert!(!b.is_superset(&a)); // b lacks 3
}

#[test]
fn set_u8_is_superset_partial_overlap_is_not_superset() {
    let a = SetU8::from([1u8, 2, 3]);
    let b = SetU8::from([2u8, 3, 4]);
    assert!(!a.is_superset(&b)); // a lacks 4
    assert!(!b.is_superset(&a)); // b lacks 1
}

#[test]
fn set_u8_is_superset_full_is_superset_of_everything() {
    assert!(SetU8::full().is_superset(&SetU8::new()));
    assert!(SetU8::full().is_superset(&SetU8::from([0u8, 128, 255])));
    assert!(SetU8::full().is_superset(&SetU8::full()));
}

#[test]
fn set_u8_is_superset_dual_of_is_subset() {
    // a.is_superset(&b) must always equal b.is_subset(&a)
    let a = SetU8::from([1u8, 2, 3]);
    let b = SetU8::from([2u8, 3, 4]);
    assert_eq!(a.is_superset(&b), b.is_subset(&a));
    assert_eq!(b.is_superset(&a), a.is_subset(&b));

    let c = SetU8::from([2u8, 3]);
    assert_eq!(a.is_superset(&c), c.is_subset(&a));
}

// ── hash ───────────────────────────────────────────────────────────────

#[test]
fn set_u8_hash() {
    #[allow(clippy::mutable_key_type)]
    let mut set = std::collections::HashSet::new();
    set.insert(SetU8::new());
    set.insert(SetU8::from([1, 2, 3]));
    set.insert(SetU8::from([1, 2, 3]));
    set.insert(SetU8::new());
    assert_eq!(set.len(), 2);
}
