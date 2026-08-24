use owo_colors::OwoColorize;
use recz_adt::{Legible, SetU8};
use recz_graph::CaptureLabel;
use std::fmt::{Display, Write};

/// Hir represents a high-level intermediate representation of a regular
/// expression, that contains bytes already encoded from unicode code points,
/// and can be used to build a graph of the corresponding finite automaton.
#[derive(Debug, Clone, PartialEq)]
pub enum Hir {
    Disjunct(DisjunctHir),
    Concat(ConcatHir),
    Repeat(RepeatHir),
    Group(GroupHir),
    Class(SetU8),
    Literal(Vec<u8>),
}

impl Hir {
    /// Creates a new disjunciton hir instance. If there is only one item, it
    /// returns that item.
    pub fn disjunct(alternatives: impl Into<Vec<Hir>>) -> Hir {
        let alters = alternatives.into();
        assert!(!alters.is_empty(), "empty disjunction is not allowed");
        if alters.len() == 1 {
            return alters.into_iter().next().unwrap();
        }
        let mut min_len = usize::MAX;
        let mut max_len = Some(0);
        for alter in &alters {
            let (alter_min_len, alter_max_len) = alter.len_hint();
            min_len = min_len.min(alter_min_len);
            max_len = if let Some(max_len) = max_len
                && let Some(alter_max_len) = alter_max_len
            {
                Some(max_len.max(alter_max_len))
            } else {
                None
            };
        }
        Hir::Disjunct(DisjunctHir {
            alters,
            min_len,
            max_len,
        })
    }

    /// Creates a new concatenation hir instance. If there is only one item, it
    /// returns that item.
    pub fn concat(items: impl Into<Vec<Hir>>) -> Hir {
        let items = items.into();
        if items.len() == 1 {
            return items.into_iter().next().unwrap();
        }
        let mut min_len = 0;
        let mut max_len = Some(0);
        for item in &items {
            let (item_min, item_max) = item.len_hint();
            min_len += item_min;
            if let Some(max) = max_len
                && let Some(item_max) = item_max
            {
                max_len = Some(max + item_max);
            } else {
                max_len = None;
            }
        }
        if items.is_empty() {
            Hir::empty()
        } else {
            Hir::Concat(ConcatHir {
                items,
                min_len,
                max_len,
            })
        }
    }

    /// Creates a new repeat hir instance.
    pub fn repeat(item: Hir, lower: usize, upper: Option<usize>) -> Hir {
        if let Some(upper) = upper {
            assert!(
                lower <= upper,
                "invalid repetition counters: {{{lower},{upper}}}"
            );
        }
        Hir::Repeat(RepeatHir {
            lower,
            upper,
            item: Box::new(item),
        })
    }

    pub fn group(label: impl Into<CaptureLabel>, item: Hir) -> Hir {
        Hir::Group(GroupHir {
            label: label.into(),
            item: Box::new(item),
        })
    }

    /// Creates a new class hir instance, i.e. a choice between possible single bytes.
    #[inline]
    pub fn class(set: SetU8) -> Hir {
        Hir::Class(set)
    }

    /// Creates a new literal hir instance, i.e. a sequence of bytes
    #[inline]
    pub fn literal(bytes: impl Into<Vec<u8>>) -> Hir {
        Hir::Literal(bytes.into())
    }

    /// Creates an empty hir instance, i.e. a literal with no bytes.
    #[inline]
    pub fn empty() -> Hir {
        Hir::Literal(vec![])
    }

    #[inline]
    pub fn is_disjunct(&self) -> bool {
        matches!(self, Hir::Disjunct(..))
    }

    #[inline]
    pub fn is_concat(&self) -> bool {
        matches!(self, Hir::Concat(..))
    }

    #[inline]
    pub fn is_repeat(&self) -> bool {
        matches!(self, Hir::Repeat(..))
    }

    #[inline]
    pub fn is_group(&self) -> bool {
        matches!(self, Hir::Group(..))
    }

    #[inline]
    pub fn is_class(&self) -> bool {
        matches!(self, Hir::Class(..))
    }

    #[inline]
    pub fn is_literal(&self) -> bool {
        matches!(self, Hir::Literal(..))
    }

    /// Returns the bounds of the Hir's length. `None` means infinite.
    pub fn len_hint(&self) -> (usize, Option<usize>) {
        match self {
            Hir::Disjunct(hir) => hir.len_hint(),
            Hir::Concat(hir) => hir.len_hint(),
            Hir::Repeat(hir) => hir.len_hint(),
            Hir::Group(hir) => hir.len_hint(),
            Hir::Class(_) => (1, Some(1)),
            Hir::Literal(bytes) => (bytes.len(), Some(bytes.len())),
        }
    }

    /// Returns `Some(len)` if this hir instance has the exact length, otherwise
    /// returns `None`.
    pub fn exact_len(&self) -> Option<usize> {
        let (lower, upper) = self.len_hint();
        if Some(lower) == upper { upper } else { None }
    }
}

impl Display for Hir {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Hir::Literal(bytes) => Display::fmt(&bytes.legible(), f),
            Hir::Class(set) => Display::fmt(set, f),
            Hir::Group(group) => Display::fmt(group, f),
            Hir::Repeat(repeat) => Display::fmt(repeat, f),
            Hir::Concat(concat) => Display::fmt(concat, f),
            Hir::Disjunct(disjunct) => Display::fmt(disjunct, f),
        }
    }
}

impl Legible for Hir {
    fn legible(&self) -> impl Display {
        self
    }

    fn colored(&self) -> impl Display {
        struct Colored<'a>(&'a Hir);
        impl Display for Colored<'_> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                match self.0 {
                    Hir::Literal(bytes) => bytes.colored().fmt(f),
                    Hir::Class(set) => set.colored().fmt(f),
                    Hir::Group(group) => group.colored().fmt(f),
                    Hir::Repeat(repeat) => repeat.colored().fmt(f),
                    Hir::Concat(concat) => concat.colored().fmt(f),
                    Hir::Disjunct(disjunct) => disjunct.colored().fmt(f),
                }
            }
        }
        Colored(self)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DisjunctHir {
    alters: Vec<Hir>,
    min_len: usize,
    max_len: Option<usize>,
}

impl DisjunctHir {
    #[inline]
    pub fn alternatives(&self) -> &[Hir] {
        &self.alters
    }

    #[inline]
    pub fn len_hint(&self) -> (usize, Option<usize>) {
        (self.min_len, self.max_len)
    }

    pub fn exact_len(&self) -> Option<usize> {
        let (lower, upper) = self.len_hint();
        if Some(lower) == upper { upper } else { None }
    }
}

impl Display for DisjunctHir {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let alters = &self.alters;
        for i in 0..alters.len() {
            if alters[i].is_concat() {
                f.write_char('(')?;
            }
            Display::fmt(&alters[i], f)?;
            if alters[i].is_concat() {
                f.write_char(')')?;
            }
            if i + 1 < alters.len() {
                f.write_str(" | ")?;
            }
        }
        Ok(())
    }
}

impl Legible for DisjunctHir {
    fn legible(&self) -> impl Display {
        self
    }

    fn colored(&self) -> impl Display {
        struct Colored<'a>(&'a DisjunctHir);
        impl Display for Colored<'_> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                for i in 0..self.0.alters.len() {
                    if self.0.alters[i].is_concat() {
                        '('.white().fmt(f)?;
                    }
                    self.0.alters[i].colored().fmt(f)?;
                    if self.0.alters[i].is_concat() {
                        ')'.white().fmt(f)?;
                    }
                    if i + 1 < self.0.alters.len() {
                        " | ".bold().white().fmt(f)?;
                    }
                }
                Ok(())
            }
        }
        Colored(self)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConcatHir {
    items: Vec<Hir>,
    min_len: usize,
    max_len: Option<usize>,
}

impl ConcatHir {
    #[inline]
    pub fn items(&self) -> &[Hir] {
        &self.items
    }

    #[inline]
    pub fn len_hint(&self) -> (usize, Option<usize>) {
        (self.min_len, self.max_len)
    }
}

impl Display for ConcatHir {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let items = &self.items;
        for i in 0..items.len() {
            if items[i].is_disjunct() {
                f.write_char('(')?;
            }
            Display::fmt(&items[i], f)?;
            if items[i].is_disjunct() {
                f.write_char(')')?;
            }
            if i + 1 < items.len() {
                f.write_str(" & ")?;
            }
        }
        Ok(())
    }
}

impl Legible for ConcatHir {
    fn legible(&self) -> impl Display {
        self
    }

    fn colored(&self) -> impl Display {
        struct Colored<'a>(&'a ConcatHir);
        impl Display for Colored<'_> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                let items = &self.0.items;
                for i in 0..items.len() {
                    if items[i].is_disjunct() {
                        '('.white().fmt(f)?;
                    }
                    items[i].colored().fmt(f)?;
                    if items[i].is_disjunct() {
                        ')'.white().fmt(f)?;
                    }
                    if i + 1 < items.len() {
                        " & ".bold().white().fmt(f)?;
                    }
                }
                Ok(())
            }
        }
        Colored(self)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RepeatHir {
    lower: usize,
    upper: Option<usize>,
    item: Box<Hir>,
}

impl RepeatHir {
    #[inline]
    pub fn inner(&self) -> &Hir {
        &self.item
    }

    pub fn len_hint(&self) -> (usize, Option<usize>) {
        let (min_len, max_len) = self.item.len_hint();
        if let Some(max) = self.upper
            && let Some(max_len) = max_len
        {
            (self.lower * min_len, Some(max * max_len))
        } else {
            (self.lower * min_len, None)
        }
    }

    /// Lower and upper bounds of possible number of iterations. `None` means infinite.
    #[inline]
    pub fn iter_hint(&self) -> (usize, Option<usize>) {
        (self.lower, self.upper)
    }
}

impl Display for RepeatHir {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let item = &self.item;
        let needs_parens = item.is_concat() || item.is_disjunct();
        if needs_parens {
            f.write_char('(')?;
        }
        Display::fmt(&item, f)?;
        if needs_parens {
            f.write_char(')')?;
        }
        match (self.lower, self.upper) {
            (0, None) => f.write_char('*'),
            (1, None) => f.write_char('+'),
            (0, Some(1)) => f.write_char('?'),
            (lower, None) => write!(f, "{{{lower},}}"),
            (lower, Some(upper)) if lower == upper => write!(f, "{{{lower}}}"),
            (lower, Some(upper)) => write!(f, "{{{lower},{upper}}}"),
        }
    }
}

impl Legible for RepeatHir {
    fn legible(&self) -> impl Display {
        self
    }

    fn colored(&self) -> impl Display {
        struct Colored<'a>(&'a RepeatHir);
        impl Display for Colored<'_> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                let item = &self.0.item;
                let needs_parens = item.is_concat() || item.is_disjunct();
                if needs_parens {
                    '('.white().fmt(f)?;
                }
                item.colored().fmt(f)?;
                if needs_parens {
                    ')'.white().fmt(f)?;
                }
                match (self.0.lower, self.0.upper) {
                    (0, None) => '*'.bold().bright_yellow().fmt(f),
                    (1, None) => '+'.bold().bright_yellow().fmt(f),
                    (0, Some(1)) => '?'.bold().bright_yellow().fmt(f),
                    (lower, None) => {
                        '{'.white().fmt(f)?;
                        lower.cyan().fmt(f)?;
                        ",}".white().fmt(f)
                    }
                    (lower, Some(upper)) if lower == upper => {
                        '{'.white().fmt(f)?;
                        lower.cyan().fmt(f)?;
                        "}".white().fmt(f)
                    }
                    (lower, Some(upper)) => {
                        '{'.white().fmt(f)?;
                        lower.cyan().fmt(f)?;
                        ','.white().fmt(f)?;
                        upper.cyan().fmt(f)?;
                        "}".white().fmt(f)
                    }
                }
            }
        }
        Colored(self)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GroupHir {
    label: CaptureLabel,
    item: Box<Hir>,
}

impl GroupHir {
    #[inline]
    pub fn inner(&self) -> &Hir {
        &self.item
    }

    #[inline]
    pub fn label(&self) -> CaptureLabel {
        self.label.clone()
    }

    #[inline]
    pub fn len_hint(&self) -> (usize, Option<usize>) {
        self.item.len_hint()
    }
}

impl Display for GroupHir {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match &self.label {
            CaptureLabel::Num(n) => write!(f, "(?D<{}> {} )", n, self.item),
            CaptureLabel::Str(s) => write!(f, "(?<{}> {} )", s, self.item),
        }
    }
}

impl Legible for GroupHir {
    fn legible(&self) -> impl Display {
        self
    }

    fn colored(&self) -> impl Display {
        struct Colored<'a>(&'a GroupHir);
        impl<'a> Display for Colored<'a> {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                "(".bold().white().fmt(f)?;
                match &self.0.label {
                    CaptureLabel::Num(n) => {
                        "?D<".white().fmt(f)?;
                        n.bold().bright_magenta().fmt(f)?;
                    }
                    CaptureLabel::Str(s) => {
                        "?<".white().fmt(f)?;
                        s.bold().bright_magenta().fmt(f)?;
                    }
                }
                "> ".white().fmt(f)?;
                self.0.item.colored().fmt(f)?;
                " )".bold().white().fmt(f)
            }
        }
        Colored(self)
    }
}
