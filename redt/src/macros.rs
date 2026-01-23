/// The macro creates a raw multiline string literal from a series of
/// documentation comments.
///
/// It allows to build strings without any escaping sequences.
#[macro_export]
macro_rules! lit {
    (#[doc=$first_line:literal] $(#[doc=$lines:literal])*) => {
        concat!($first_line, $("\n", $lines),*)
    };
}

/// The macro creates a set from a series of elements separated by commas.
#[macro_export]
macro_rules! set {
    ($($elem:expr),* $(,)?) => {
        {
            #[allow(clippy::mutable_key_type)]
            let mut set = ::redt::Set::default();
            $(set.insert($elem);)*
            set
        }
    };
}

/// The macro creates a map from a series of key-value pairs separated by
/// commas.
#[macro_export]
macro_rules! map {
    ($($key:expr => $value:expr),* $(,)?) => {
        {
            #[allow(clippy::mutable_key_type)]
            let mut map = ::redt::Map::default();
            $(map.insert($key, $value);)*
            map
        }
    };
}
