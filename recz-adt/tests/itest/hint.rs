use pretty_assertions::assert_eq;
use recz_adt::LenHint;

#[test]
fn len_hint_new() {
    let hint = LenHint::new(0, Some(0));
    assert_eq!(hint.min(), 0);
    assert_eq!(hint.max(), Some(0));

    let hint = LenHint::new(2, Some(5));
    assert_eq!(hint.min(), 2);
    assert_eq!(hint.max(), Some(5));

    let hint = LenHint::new(3, None);
    assert_eq!(hint.min(), 3);
    assert_eq!(hint.max(), None);

    let hint = LenHint::new(usize::MAX - 1, Some(usize::MAX - 1));
    assert_eq!(hint.min(), usize::MAX - 1);
    assert_eq!(hint.max(), Some(usize::MAX - 1));
}

#[test]
#[should_panic]
fn len_hint_new_min_overflow() {
    _ = LenHint::new(usize::MAX, None);
}

#[test]
#[should_panic]
fn len_hint_new_max_overflow() {
    _ = LenHint::new(0, Some(usize::MAX));
}

#[test]
fn len_hint_exact_len() {
    assert_eq!(LenHint::new(0, Some(0)).exact_len(), Some(0));
    assert_eq!(LenHint::new(4, Some(4)).exact_len(), Some(4));
    assert_eq!(LenHint::new(4, Some(5)).exact_len(), None);
    assert_eq!(LenHint::new(4, None).exact_len(), None);
}

#[test]
fn len_hint_to_tuple() {
    assert_eq!(LenHint::new(1, Some(2)).to_tuple(), (1, Some(2)));
    assert_eq!(LenHint::new(7, None).to_tuple(), (7, None));
}

#[test]
fn len_hint_eq() {
    assert_eq!(LenHint::new(1, Some(2)), LenHint::new(1, Some(2)));
    assert_ne!(LenHint::new(1, Some(2)), LenHint::new(1, Some(3)));
    assert_ne!(LenHint::new(1, Some(2)), LenHint::new(1, None));
    assert_ne!(LenHint::new(0, None), LenHint::new(1, None));

    assert!(LenHint::new(1, Some(2)) == (1, Some(2)));
    assert!(LenHint::new(1, None) == (1, None));
    assert!(LenHint::new(1, None) != (1, Some(1)));
}

#[test]
fn len_hint_debug_fmt() {
    assert_eq!(format!("{:?}", LenHint::new(1, Some(2))), "(1, Some(2))");
    assert_eq!(format!("{:?}", LenHint::new(0, None)), "(0, None)");
}

#[test]
fn len_hint_bitand() {
    assert_eq!(
        LenHint::new(1, Some(2)) & LenHint::new(3, Some(4)),
        (4, Some(6))
    );
    assert_eq!(
        LenHint::new(0, Some(0)) & LenHint::new(3, Some(4)),
        (3, Some(4))
    );
    assert_eq!(LenHint::new(1, None) & LenHint::new(3, Some(4)), (4, None));
    assert_eq!(LenHint::new(1, Some(2)) & LenHint::new(3, None), (4, None));
    assert_eq!(LenHint::new(1, None) & LenHint::new(3, None), (4, None));

    let mut hint = LenHint::new(1, Some(1));
    hint &= LenHint::new(2, Some(3));
    assert_eq!(hint, (3, Some(4)));
    hint &= LenHint::new(0, None);
    assert_eq!(hint, (3, None));
}

#[test]
#[should_panic]
fn len_hint_bitand_min_overflow() {
    _ = LenHint::new(usize::MAX - 1, None) & LenHint::new(1, None);
}

#[test]
#[should_panic]
fn len_hint_bitand_max_overflow() {
    _ = LenHint::new(0, Some(usize::MAX - 1)) & LenHint::new(0, Some(1));
}

#[test]
fn len_hint_bitor() {
    assert_eq!(
        LenHint::new(1, Some(2)) | LenHint::new(3, Some(4)),
        (1, Some(4))
    );
    assert_eq!(
        LenHint::new(3, Some(4)) | LenHint::new(1, Some(2)),
        (1, Some(4))
    );
    assert_eq!(
        LenHint::new(2, Some(8)) | LenHint::new(3, Some(4)),
        (2, Some(8))
    );
    assert_eq!(LenHint::new(1, None) | LenHint::new(3, Some(4)), (1, None));
    assert_eq!(LenHint::new(1, Some(2)) | LenHint::new(0, None), (0, None));

    let mut hint = LenHint::new(5, Some(5));
    hint |= LenHint::new(2, Some(3));
    assert_eq!(hint, (2, Some(5)));
    hint |= LenHint::new(4, None);
    assert_eq!(hint, (2, None));
}

#[test]
#[allow(clippy::erasing_op)]
fn len_hint_mul_usize() {
    assert_eq!(LenHint::new(1, Some(2)) * 3, (3, Some(6)));
    assert_eq!(LenHint::new(2, None) * 3, (6, None));
    assert_eq!(LenHint::new(1, Some(2)) * 0, (0, Some(0)));
    assert_eq!(LenHint::new(1, Some(2)) * 1, (1, Some(2)));

    let mut hint = LenHint::new(2, Some(3));
    hint *= 2;
    assert_eq!(hint, (4, Some(6)));
}

#[test]
fn len_hint_mul() {
    assert_eq!(
        LenHint::new(1, Some(2)) * LenHint::new(3, Some(4)),
        (3, Some(8))
    );
    assert_eq!(
        LenHint::new(2, Some(2)) * LenHint::new(0, Some(0)),
        (0, Some(0))
    );
    assert_eq!(LenHint::new(1, Some(2)) * LenHint::new(3, None), (3, None));
    assert_eq!(LenHint::new(1, None) * LenHint::new(3, Some(4)), (3, None));
}
