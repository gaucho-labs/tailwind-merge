mod color;

pub use self::color::TailwindColor;
use crate::{syntax_error, Result, TailwindArbitrary};
use css_color::Srgb;
use std::fmt::{Display, Formatter};
