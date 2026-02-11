#![allow(dead_code)]

use super::color::{bg_rgb, fg_rgb};

pub fn fg() -> String {
    fg_rgb(230, 230, 230) // #e6e6e6
}

pub fn bg() -> String {
    bg_rgb(17, 17, 17) // #111111
}

pub fn rust() -> String {
    fg_rgb(196, 111, 74) // #c46f4a
}

pub fn error() -> String {
    fg_rgb(224, 108, 117) // #e06c75
}

pub fn success() -> String {
    fg_rgb(152, 195, 121) // #98c379
}

pub fn path() -> String {
    fg_rgb(97, 175, 239) // #61afef
}
