mod codec;
pub use codec::Codec;

mod error;
pub use error::{Error, Result};

mod encoding;
pub use encoding::Encoding;

mod utf8;
pub use utf8::Utf8Codec;
