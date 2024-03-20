#![allow(dead_code)]

use std::fmt::Debug;
use std::sync::{Arc, Mutex};

use image::{Image, ImageData};
use positioner::{Row, Section, Spacer};
use table::Table;
use text::TextBox;

mod clipboard;
pub mod color;
mod debug_impls;
pub mod fonts;
pub mod history;
pub mod image;
pub mod interpreter;
mod keybindings;
pub mod opts;
pub mod positioner;
// pub mod renderer;
pub mod table;
pub mod test_utils;
pub mod text;
pub mod utils;

pub use utils::ImageCache;

pub enum InlyneEvent {
    LoadedImage(String, Arc<Mutex<Option<ImageData>>>),
    FileReload,
    FileChange { contents: String },
    Reposition,
    PositionQueue,
}

impl Debug for InlyneEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Inlyne Event")
    }
}

#[derive(Debug)]
pub enum Element {
    TextBox(TextBox),
    Spacer(Spacer),
    Image(Image),
    Table(Table),
    Row(Row),
    Section(Section),
}

impl From<Section> for Element {
    fn from(section: Section) -> Self {
        Element::Section(section)
    }
}

impl From<Row> for Element {
    fn from(row: Row) -> Self {
        Element::Row(row)
    }
}

impl From<Image> for Element {
    fn from(image: Image) -> Self {
        Element::Image(image)
    }
}

impl From<Spacer> for Element {
    fn from(spacer: Spacer) -> Self {
        Element::Spacer(spacer)
    }
}

impl From<TextBox> for Element {
    fn from(text_box: TextBox) -> Self {
        Element::TextBox(text_box)
    }
}

impl From<Table> for Element {
    fn from(table: Table) -> Self {
        Element::Table(table)
    }
}
