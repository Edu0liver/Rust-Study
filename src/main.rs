#![allow(dead_code)]
#![allow(unused_imports)]

mod guessing_game;
use crate::guessing_game::game;

mod struct_try;
use crate::struct_try::struct_try;

mod enum_try;
use crate::enum_try::enum_try;

fn main() {
    enum_try();
}
