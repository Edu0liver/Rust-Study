#![allow(unused_imports, dead_code)]

mod choice;
use crate::choice::choice::choice;

mod guessing_game;
use crate::guessing_game::game;

mod struct_try;
use crate::struct_try::struct_try;

mod enum_try;
use crate::enum_try::enum_try;

mod generic_try;
use crate::generic_try::generic_try;

mod traits_try;
use crate::traits_try::traits_try;

fn main() {
    choice();
}
