//! Defines constants that expand to terminal control codes for color and style manipulation
//! when used in an interactive terminal session.
//! If the output is piped to another program or into a file this will be detected with the
//! [`atty`](https://docs.rs/atty/latest/atty/) crate and the constants will instead expand to
//! blank strings.
//!
//! The constants have type `&'static str` and then get transformed by [`lazy_static`](https://docs.rs/lazy_static/latest/lazy_static/)
//!

use lazy_static::lazy_static;

fn color(t: &'static str) -> &'static str {
    if atty::is(atty::Stream::Stdout) {
        t
    } else {
        ""
    }
}

macro_rules! colors {
    ($($n: ident, $s: literal);* $(;)?) => {
        $(colors!{single: $n, $s})*
    };
    (single: $name: ident, $seq: literal) => {
        lazy_static! {
            #[doc = concat!("**", stringify!{$name}, "**:")]
            #[doc = " terminal marker, either blank string or"]
            #[doc = concat!("**", stringify!{$seq}, "**")]
            pub static ref $name: &'static str = color($seq);
        }
        impl $name {
            pub fn only_if(&self, predicate: impl Into<bool>) -> &str {
                if predicate.into() { self } else { "" }
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let s: &str = std::ops::Deref::deref(&self);
                f.write_str(s)
            }
        }
    };

}

colors!(
    COLOR_RESET, "\x1B[39m";
    COLOR_BLACK, "\x1B[30m";
    COLOR_RED, "\x1B[31m";
    COLOR_GREEN, "\x1B[32m";
    COLOR_YELLOW, "\x1B[33m";
    COLOR_BLUE, "\x1B[34m";
    COLOR_MAGENTA, "\x1B[35m";
    COLOR_CYAN, "\x1B[36m";
    COLOR_WHITE, "\x1B[37m";
    COLOR_BRIGHT_BLACK, "\x1B[90m";
    COLOR_BRIGHT_RED, "\x1B[91m";
    COLOR_BRIGHT_GREEN, "\x1B[92m";
    COLOR_BRIGHT_YELLOW, "\x1B[93m";
    COLOR_BRIGHT_BLUE, "\x1B[94m";
    COLOR_BRIGHT_MAGENTA, "\x1B[95m";
    COLOR_BRIGHT_CYAN, "\x1B[96m";
    COLOR_BRIGHT_WHITE, "\x1B[97m";

    BG_BLACK, "\x1B[40m";
    BG_RED, "\x1B[41m";
    BG_GREEN, "\x1B[42m";
    BG_YELLOW, "\x1B[43m";
    BG_BLUE, "\x1B[44m";
    BG_MAGENTA, "\x1B[45m";
    BG_CYAN, "\x1B[46m";
    BG_WHITE, "\x1B[47m";
    BG_BRIGHT_BLACK, "\x1B[100m";
    BG_BRIGHT_RED, "\x1B[101m";
    BG_BRIGHT_GREEN, "\x1B[102m";
    BG_BRIGHT_YELLOW, "\x1B[103m";
    BG_BRIGHT_BLUE, "\x1B[104m";
    BG_BRIGHT_MAGENTA, "\x1B[105m";
    BG_BRIGHT_CYAN, "\x1B[106m";
    BG_BRIGHT_WHITE, "\x1B[107m";
    BG_RESET, "\x1B[49m";

    STYLE_BOLD, "\x1B[1m";
    STYLE_UNDERLINE, "\x1B[4m";
    STYLE_RESET, "\x1B[0m";
);
