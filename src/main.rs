// Standard Imports
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::io::prelude::*;
use std::path::Path;

// Cursive Imports
use cursive::Cursive;
use cursive::views::{Button, Dialog, DummyView, EditView, LinearLayout, SelectView, TextView};
use cursive::traits::*;

fn main() {
    let mut siv = Cursive::default();

    siv.run();
}
