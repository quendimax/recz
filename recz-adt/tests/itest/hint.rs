use pretty_assertions::assert_eq;
use recz_adt::SizeHint;

#[test]
fn len_hint_new() {
    let hint = SizeHint::new(0, Some(0));
    assert_eq!(hint.least(), 0);
    assert_eq!(hint.most(), Some(0));

    let hint = SizeHint::new(2, Some(5));
    assert_eq!(hint.least(), 2);
    assert_eq!(hint.most(), Some(5));

    let hint = SizeHint::new(3, None);
    assert_eq!(hint.least(), 3);
    assert_eq!(hint.most(), None);

    let hint = SizeHint::new(usize::MAX - 1, Some(usize::MAX - 1));
    assert_eq!(hint.least(), usize::MAX - 1);
    assert_eq!(hint.most(), Some(usize::MAX - 1));
}

#[test]
#[should_panic]
fn len_hint_new_min_overflow() {
    _ = SizeHint::new(usize::MAX, None);
}

#[test]
#[should_panic]
fn len_hint_new_max_overflow() {
    _ = SizeHint::new(0, Some(usize::MAX));
}

#[test]
fn len_hint_exact_len() {
    assert_eq!(SizeHint::new(0, Some(0)).exact_size(), Some(0));
    assert_eq!(SizeHint::new(4, Some(4)).exact_size(), Some(4));
    assert_eq!(SizeHint::new(4, Some(5)).exact_size(), None);
    assert_eq!(SizeHint::new(4, None).exact_size(), None);
}

#[test]
fn len_hint_to_tuple() {
    assert_eq!(SizeHint::new(1, Some(2)).to_tuple(), (1, Some(2)));
    assert_eq!(SizeHint::new(7, None).to_tuple(), (7, None));
}

#[test]
fn len_hint_eq() {
    assert_eq!(SizeHint::new(1, Some(2)), SizeHint::new(1, Some(2)));
    assert_ne!(SizeHint::new(1, Some(2)), SizeHint::new(1, Some(3)));
    assert_ne!(SizeHint::new(1, Some(2)), SizeHint::new(1, None));
    assert_ne!(SizeHint::new(0, None), SizeHint::new(1, None));

    assert!(SizeHint::new(1, Some(2)) == (1, Some(2)));
    assert!(SizeHint::new(1, None) == (1, None));
    assert!(SizeHint::new(1, None) != (1, Some(1)));
}

#[test]
fn len_hint_debug_fmt() {
    assert_eq!(format!("{:?}", SizeHint::new(1, Some(2))), "(1, Some(2))");
    assert_eq!(format!("{:?}", SizeHint::new(0, None)), "(0, None)");
}

#[test]
fn len_hint_bitand() {
    assert_eq!(
        SizeHint::new(1, Some(2)) & SizeHint::new(3, Some(4)),
        (4, Some(6))
    );
    assert_eq!(
        SizeHint::new(0, Some(0)) & SizeHint::new(3, Some(4)),
        (3, Some(4))
    );
    assert_eq!(
        SizeHint::new(1, None) & SizeHint::new(3, Some(4)),
        (4, None)
    );
    assert_eq!(
        SizeHint::new(1, Some(2)) & SizeHint::new(3, None),
        (4, None)
    );
    assert_eq!(SizeHint::new(1, None) & SizeHint::new(3, None), (4, None));

    let mut hint = SizeHint::new(1, Some(1));
    hint &= SizeHint::new(2, Some(3));
    assert_eq!(hint, (3, Some(4)));
    hint &= SizeHint::new(0, None);
    assert_eq!(hint, (3, None));
}

#[test]
#[should_panic]
fn len_hint_bitand_min_overflow() {
    _ = SizeHint::new(usize::MAX - 1, None) & SizeHint::new(1, None);
}

#[test]
#[should_panic]
fn len_hint_bitand_max_overflow() {
    _ = SizeHint::new(0, Some(usize::MAX - 1)) & SizeHint::new(0, Some(1));
}

#[test]
fn len_hint_bitor() {
    assert_eq!(
        SizeHint::new(1, Some(2)) | SizeHint::new(3, Some(4)),
        (1, Some(4))
    );
    assert_eq!(
        SizeHint::new(3, Some(4)) | SizeHint::new(1, Some(2)),
        (1, Some(4))
    );
    assert_eq!(
        SizeHint::new(2, Some(8)) | SizeHint::new(3, Some(4)),
        (2, Some(8))
    );
    assert_eq!(
        SizeHint::new(1, None) | SizeHint::new(3, Some(4)),
        (1, None)
    );
    assert_eq!(
        SizeHint::new(1, Some(2)) | SizeHint::new(0, None),
        (0, None)
    );

    let mut hint = SizeHint::new(5, Some(5));
    hint |= SizeHint::new(2, Some(3));
    assert_eq!(hint, (2, Some(5)));
    hint |= SizeHint::new(4, None);
    assert_eq!(hint, (2, None));
}

#[test]
#[allow(clippy::erasing_op)]
fn len_hint_mul_usize() {
    assert_eq!(SizeHint::new(1, Some(2)) * 3, (3, Some(6)));
    assert_eq!(SizeHint::new(2, None) * 3, (6, None));
    assert_eq!(SizeHint::new(1, Some(2)) * 0, (0, Some(0)));
    assert_eq!(SizeHint::new(1, Some(2)) * 1, (1, Some(2)));

    let mut hint = SizeHint::new(2, Some(3));
    hint *= 2;
    assert_eq!(hint, (4, Some(6)));

    let mut hint = SizeHint::new(2, Some(3));
    let multiplier = SizeHint::new(3, Some(4));
    hint *= multiplier;
    assert_eq!(hint, (6, Some(12)));
}

#[test]
fn len_hint_mul() {
    assert_eq!(
        SizeHint::new(1, Some(2)) * SizeHint::new(3, Some(4)),
        (3, Some(8))
    );
    assert_eq!(
        SizeHint::new(2, Some(2)) * SizeHint::new(0, Some(0)),
        (0, Some(0))
    );
    assert_eq!(
        SizeHint::new(1, Some(2)) * SizeHint::new(3, None),
        (3, None)
    );
    assert_eq!(
        SizeHint::new(1, None) * SizeHint::new(3, Some(4)),
        (3, None)
    );
}
