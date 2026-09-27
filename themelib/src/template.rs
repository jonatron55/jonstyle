use std::{
    collections::HashSet,
    error::Error as StdError,
    f64::consts::TAU,
    fmt::{Display, Formatter, Result as FmtResult},
    fs::File,
    io::{Read, Result as IoResult, Write},
    iter,
    path::Path,
};

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

use crate::{
    color::OkHsla,
    theme::{Indexer, Theme, ThemeVariant},
};

/// Defines a template for generating theme variants. A template is a  plain
/// text file containing placeholders for theme values in the format
/// "{{<field>}}", with optional formatting options. The formatting options
/// depend on the type of the field, which may be a string or a color.
///
/// String fields take the format "{{<field>[:<format>]}}", where <field> is one
/// of the following:
///
/// - "name": the name of the theme.
/// - "variant": the name of the theme variant, ("dawn", "dusk", "noon", or
///   "night").
/// - "author": the author of the theme.
/// - "description": the description of the theme.
/// - "version": the version of the theme.
///
/// And <format> is an optional format specifier for the field, which can be one
/// of:
///
/// - "c": lowerCamelCase.
/// - "P" or "C": PascalCase (a.k.a. UpperCamelCase).
/// - "s": lower_snake_case.
/// - "S": Upper_Snake_Case.
/// - "SS": SCREAMING_SNAKE_CASE.
/// - "k": lower-kebab-case.
/// - "K": Upper-Kebab-Case.
/// - "SK": SCREAMING-KEBAB-CASE.
/// - "U": UPPERCASESINGLEWORD.
/// - "T": Title Case (capitalized words separated by spaces).
/// - "t": Sentence case (first word capitalized separated by spaces).
/// - "l": lowercasesingleword.
/// - "V", "v", or absent: the original value verbatim.
///
/// Color fields take the format
/// `{{<field>[.<swizzle>[/<colorspace]][:<format>]}}`. The field name may be
/// any of the options documented in [`themelib::theme::Indexer`]. The optional
/// <swizzle> is a string of the characters 'r', 'g', 'b', 'h', 's', and 'l' in
/// any order, which specifies the order of color components in the output. The
/// optional <colorspace> is either 'G' for gamma-corrected (sRGB) values or 'L'
/// for linear values, which affects the output values for 'r', 'g', and 'b'
/// components. The optional <format> may be one of the following:
///
/// - "X": hexadecimal RGB format, with uppercase letters and no prefix. This is
///   the default format if none is specified.
/// - "x": hexadecimal RGB format, with lowercase letters and no prefix.
/// - "V", "v", "F", or "f": a comma-separated list of real-numbered components
///   in the range 0.0 to 1.0.
/// - "E": A comma-separated list of components in exponential notation with an
///   uppercase 'E'.
/// - "e": A comma-separated list of components in exponential notation with a
///   lowercase 'e'.
/// - "B" or "b": a comma-separated list of byte components in the range 0 to
///   255.
/// - "P": a comma-separated list of percentage components with a '%' symbol.
/// - "p": a comma-separated list of percentage components without a symbol.
/// - "D" or "d": a comma-separated list of degree components in the range 0 to
///   360.
/// - "R" or "r": a comma-separated list of radian components in the range 0 to
///   2π.
///
/// For example, the placeholder "{{primary-foreground.hsl.s:P}}" might be
/// replaced with "80%" or "{{base-color-1-high-medium.rgb:X}}" might be
/// replaced with "A0C8FF".
///
/// Finally, the sequences "{{left}}" and "{{right}}" can be used to insert
/// literal "{{" and "}}" sequences, respectively.
pub struct Template {
    source: String,
}

lazy_static! {
    static ref TEMPLATE_REGEX: Regex =
        Regex::new(r"\{\{([\$A-Za-z0-9\-_]+)(?:\.([A-Za-z]+))?(?:/([A-Za-z]+))?(?::([A-Za-z]+))?\}\}").unwrap();
    static ref LOOP_REGEX: Regex = Regex::new(r"(?s)\{\{#each(\s+[A-Za-z0-9\-_]+)?\}\}(.*)\{\{\/each\}\}").unwrap();
    static ref PREPS: HashSet<&'static str> = {
        let mut set = HashSet::new();
        set.insert("a");
        set.insert("an");
        set.insert("the");
        set.insert("and");
        set.insert("or");
        set.insert("but");
        set.insert("for");
        set.insert("nor");
        set.insert("of");
        set.insert("on");
        set.insert("in");
        set.insert("at");
        set.insert("to");
        set.insert("by");
        set
    };
}

#[derive(Debug, Clone)]
pub enum Error {
    NotFound(String),
    BadFormat(String),
    BadColorSpace(String),
    BadSwizzle(String),
    InvalidIterator(String),
    InvalidContext(String),
}

impl Template {
    pub fn new(source: impl Into<String>) -> Self {
        let source = source.into();
        Self { source }
    }

    pub fn read<R: Read>(mut reader: R) -> IoResult<Self> {
        let mut buf = String::new();
        reader.read_to_string(&mut buf)?;
        Ok(Self { source: buf })
    }

    pub fn from_file(path: &Path) -> IoResult<Self> {
        let file = File::open(path)?;
        Self::read(file)
    }

    pub fn render_to_string(&self, context: &Theme, variant: ThemeVariant) -> (String, Vec<Error>) {
        let mut output = Vec::with_capacity(self.source.len());
        let errors = self.render(context, variant, &mut output).unwrap();
        (String::from_utf8(output).unwrap(), errors)
    }

    pub fn render(&self, context: &Theme, variant: ThemeVariant, writer: &mut impl Write) -> IoResult<Vec<Error>> {
        let Self { source } = self;
        let mut errors = Vec::new();

        Self::expand_loops(&source, context, variant, writer, &mut errors)?;

        Ok(errors)
    }

    fn expand_loops(
        source: impl AsRef<str>,
        context: &Theme,
        variant: ThemeVariant,
        writer: &mut impl Write,
        errors: &mut Vec<Error>,
    ) -> IoResult<()> {
        let mut last_index = 0;

        for cap in LOOP_REGEX.captures_iter(source.as_ref()) {
            let whole_match = cap.get(0).unwrap();

            Self::render_inner(
                &source.as_ref()[last_index..whole_match.start()],
                context,
                variant,
                None,
                None,
                writer,
                errors,
            )?;

            let iter: Box<dyn Iterator<Item = Indexer>> = match cap.get(1).map(|m| m.as_str().trim()) {
                Some("base") => Box::new(Indexer::iter_base()),
                Some("themed") => Box::new(Indexer::iter_themed(variant)),
                Some("primary") => Box::new(Indexer::iter_primary()),
                Some("themed-primary") => Box::new(Indexer::iter_themed_primary(variant)),
                Some("semantic") => Box::new(Indexer::iter_semantic(variant)),
                Some(other) if other.is_empty() => Box::new(Indexer::iter(variant)),
                Some(other) => {
                    write!(std::io::stderr(), "<<INVALID ITERATOR: '{other}'>>").unwrap();
                    Box::new(iter::empty())
                }
                None => Box::new(Indexer::iter(variant)),
            };

            let inner_source = cap.get(2).unwrap().as_str();

            for indexer in iter {
                let key = format!("{indexer}");
                let value = context.get(&indexer);
                Self::render_inner(inner_source, context, variant, Some(&key), Some(&value), writer, errors)?;
            }

            last_index = whole_match.end();
        }

        Self::render_inner(
            &source.as_ref()[last_index..],
            context,
            variant,
            None,
            None,
            writer,
            errors,
        )?;
        Ok(())
    }

    fn render_inner(
        source: impl AsRef<str>,
        context: &Theme,
        variant: ThemeVariant,
        key: Option<&str>,
        value: Option<&OkHsla>,
        writer: &mut impl Write,
        errors: &mut Vec<Error>,
    ) -> IoResult<()> {
        let source = source.as_ref();
        let mut last_index = 0;

        for cap in TEMPLATE_REGEX.captures_iter(&source) {
            writer.write_all(&source[last_index..cap.get(0).unwrap().start()].as_bytes())?;

            let Some(prop) = cap.get(1).map(|m| m.as_str()) else {
                continue;
            };
            let swizzle = cap.get(2).map(|m| m.as_str());
            let space = cap.get(3).map(|m| m.as_str());
            let fmt = cap.get(4).map(|m| m.as_str());

            match prop {
                "left" => {
                    writer.write_all(b"{{")?;
                }
                "right" => {
                    writer.write_all(b"}}")?;
                }
                "name" => {
                    write_string(writer, &context.name, fmt, errors)?;
                }
                "author" => {
                    if let Some(author) = &context.author {
                        write_string(writer, author, fmt, errors)?;
                    }
                }
                "description" => {
                    if let Some(description) = &context.description {
                        write_string(writer, description, fmt, errors)?;
                    }
                }
                "version" => {
                    write_string(writer, &context.version.to_string(), fmt, errors)?;
                }
                "variant" => {
                    write_string(writer, &variant.to_string(), fmt, errors)?;
                }
                "mode" => {
                    write_string(writer, &variant.mode.to_string(), fmt, errors)?;
                }
                "temperature" => {
                    write_string(writer, &variant.temperature.to_string(), fmt, errors)?;
                }
                "$key" => {
                    if let Some(key) = key {
                        write_string(writer, key, fmt, errors)?;
                    } else {
                        errors.push(Error::InvalidContext("$key".to_string()));
                        write!(writer, "<<INVALID CONTEXT: '$key'>>")?;
                    }
                }
                "$value" => {
                    if let Some(value) = value {
                        write_color(writer, value, swizzle, space, fmt, errors)?;
                    } else {
                        errors.push(Error::InvalidContext("$value".to_string()));
                        write!(writer, "<<INVALID CONTEXT: '$value'>>")?;
                    }
                }
                other => {
                    if let Ok(indexer) = Indexer::from_str_with_variant(other, variant) {
                        write_color(writer, &context.get(&indexer), swizzle, space, fmt, errors)?
                    } else {
                        errors.push(Error::NotFound(other.to_string()));
                        write!(writer, "<<NOT FOUND: '{other}'>>")?;
                    }
                }
            };

            last_index = cap.get(0).unwrap().end();
        }

        writer.write_all(&source[last_index..].as_bytes())?;

        Ok(())
    }
}

pub fn write_string(writer: &mut impl Write, name: &str, fmt: Option<&str>, errors: &mut Vec<Error>) -> IoResult<()> {
    let mut parts = name.split(|c| c == '-' || c == '_' || c == ' ').map(|part| part.to_lowercase());

    let capitalize = |s: String| {
        let mut chars = s.chars();
        match chars.next() {
            Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            None => String::new(),
        }
    };

    #[allow(unstable_name_collisions)]
    match fmt {
        Some("P") | Some("C") => {
            // PascalCase or capital CamelCase
            for part in parts.map(capitalize) {
                writer.write_all(part.as_bytes())?;
            }
        }
        Some("c") => {
            // camelCase
            if let Some(first) = parts.next() {
                writer.write_all(first.to_lowercase().as_bytes())?;
            }

            for part in parts.map(capitalize) {
                writer.write_all(part.as_bytes())?;
            }
        }
        Some("s") => {
            // snake_case
            for part in parts.map(|part| part.to_lowercase()).intersperse("_".to_string()) {
                writer.write_all(part.as_bytes())?;
            }
        }
        Some("S") => {
            // Snake_Case
            for part in parts.map(capitalize).intersperse("_".to_string()) {
                writer.write_all(part.as_bytes())?;
            }
        }
        Some("SS") => {
            // SCREAMING_SNAKE_CASE
            for part in parts.map(|part| part.to_uppercase()).intersperse("_".to_string()) {
                writer.write_all(part.as_bytes())?;
            }
        }
        Some("k") => {
            // kebab-case
            for part in parts.map(|part| part.to_lowercase()).intersperse("-".to_string()) {
                writer.write_all(part.as_bytes())?;
            }
        }
        Some("K") => {
            // Kebab-Case
            for part in parts.map(capitalize).intersperse("-".to_string()) {
                writer.write_all(part.as_bytes())?;
            }
        }
        Some("SK") => {
            // SCREAMING-KEBAB-CASE
            for part in parts.map(|part| part.to_uppercase()).intersperse("-".to_string()) {
                writer.write_all(part.as_bytes())?;
            }
        }
        Some("U") => {
            // UPPERCASE
            for part in parts.map(|part| part.to_uppercase()) {
                writer.write_all(part.as_bytes())?;
            }
        }
        Some("T") => {
            // Title Case
            if let Some(first) = parts.next() {
                writer.write_all(capitalize(first).as_bytes())?;
            }

            for part in parts.map(|part| {
                if PREPS.contains(part.as_str()) {
                    part.to_lowercase()
                } else {
                    capitalize(part)
                }
            }) {
                writer.write_all(part.as_bytes())?;
            }
        }
        Some("t") => {
            // Title case
            if let Some(first) = parts.next() {
                writer.write_all(capitalize(first).as_bytes())?;
            }
            for part in parts.map(|part| part.to_lowercase()) {
                writer.write_all(part.as_bytes())?;
            }
        }
        Some("l") => {
            // lowercase
            for part in parts.map(|part| part.to_lowercase()) {
                writer.write_all(part.as_bytes())?;
            }
        }
        Some("v") | Some("V") => {
            // Verbatim
            writer.write_all(name.as_bytes())?;
        }
        Some(other) => {
            errors.push(Error::BadFormat(other.to_string()));
            write!(writer, "<<BAD FORMAT: '{other}'>>")?;
        }
        None => writer.write_all(name.as_bytes())?,
    }

    Ok(())
}

pub fn write_color<W: Write>(
    writer: &mut W,
    value: &OkHsla,
    order: Option<&str>,
    space: Option<&str>,
    fmt: Option<&str>,
    errors: &mut Vec<Error>,
) -> IoResult<()> {
    let gamma = match space {
        Some("G") | Some("g") | None => true,
        Some("L") | Some("l") => false,
        Some(other) => {
            errors.push(Error::BadColorSpace(other.to_string()));
            write!(writer, "<<BAD COLOR SPACE: '{other}'>>")?;
            false
        }
    };

    let Ok(components) = swizzle(*value, order, gamma) else {
        errors.push(Error::BadSwizzle(order.unwrap().to_string()));
        return write!(writer, "<<BAD SWIZZLE: '{}'>>", order.unwrap());
    };

    let to_byte = |f: f64| (f.clamp(0.0, 1.0) * 255.0).round() as u8;

    match fmt {
        // Lowercase hexadecimal
        Some("x") => {
            for c in components {
                write!(writer, "{:02x}", to_byte(c))?;
            }
        }

        // Uppercase hexadecimal (default)
        Some("X") | None => {
            for c in components {
                write!(writer, "{:02X}", to_byte(c))?;
            }
        }

        // Vector of floats
        Some("V") | Some("v") | Some("F") | Some("f") => {
            for (i, c) in components.iter().enumerate() {
                if i > 0 {
                    write!(writer, ", ")?;
                }
                write!(writer, "{c}")?;
            }
        }

        // Uppercase exponential notation
        Some("E") => {
            for (i, &c) in components.iter().enumerate() {
                if i > 0 {
                    write!(writer, ", ")?;
                }
                write!(writer, "{c:E}")?;
            }
        }

        // Lowercase exponential notation
        Some("e") => {
            for (i, &c) in components.iter().enumerate() {
                if i > 0 {
                    write!(writer, ", ")?;
                }
                write!(writer, "{c:e}")?;
            }
        }

        // Bytes
        Some("B") | Some("b") => {
            for (i, &c) in components.iter().enumerate() {
                if i > 0 {
                    write!(writer, ", ")?;
                }
                write!(writer, "{}", to_byte(c))?;
            }
        }

        // Percentage with symbol
        Some("P") => {
            for (i, &c) in components.iter().enumerate() {
                if i > 0 {
                    write!(writer, ", ")?;
                }
                write!(writer, "{}%", (c.clamp(0.0, 1.0) * 100.0).round())?;
            }
        }

        // Percentage without symbol
        Some("p") => {
            for (i, &c) in components.iter().enumerate() {
                if i > 0 {
                    write!(writer, ", ")?;
                }
                write!(writer, "{}", (c.clamp(0.0, 1.0) * 100.0).round())?;
            }
        }

        // Degrees
        Some("D") | Some("d") => {
            for (i, &c) in components.iter().enumerate() {
                if i > 0 {
                    write!(writer, ", ")?;
                }
                write!(writer, "{}", (c * 360.0).round())?;
            }
        }

        // Radians
        Some("R") | Some("r") => {
            for (i, &c) in components.iter().enumerate() {
                if i > 0 {
                    write!(writer, ", ")?;
                }
                write!(writer, "{}", c * TAU)?;
            }
        }

        Some(other) => {
            errors.push(Error::BadFormat(other.to_string()));
            write!(writer, "<<BAD FORMAT: '{other}'>>")?;
        }
    }

    Ok(())
}

pub fn fmt_string(name: impl AsRef<str>, fmt: impl AsRef<str>) -> String {
    let name = name.as_ref();
    let fmt = fmt.as_ref();
    let mut w = Vec::with_capacity(name.len());
    let mut errors = vec![];

    match write_string(&mut w, name, Some(fmt), &mut errors) {
        Ok(()) => String::from_utf8_lossy(&w).to_string(),
        Err(_) => errors.iter().map(|err| err.to_string()).join(" "),
    }
}

pub fn fmt_color(value: &OkHsla, fmt: &str, order: &str, linear: bool) -> String {
    let mut w = vec![];
    let mut errors = vec![];
    match write_color(
        &mut w,
        value,
        Some(fmt),
        Some(order),
        Some(if linear { "L" } else { "G" }),
        &mut errors,
    ) {
        Ok(()) => String::from_utf8_lossy(&w).to_string(),
        Err(_) => errors.iter().map(|err| err.to_string()).join(" "),
    }
}

fn swizzle(color: OkHsla, order: Option<&str>, gamma: bool) -> Result<Vec<f64>, ()> {
    let OkHsla { h, s, l, a } = color;
    let [r, g, b, _] = if gamma {
        color.to_srgba().as_array()
    } else {
        color.to_rgba().as_array()
    };

    let order = match order {
        Some(o) => o,
        None => return Ok(vec![r, g, b]),
    };

    let mut result = Vec::with_capacity(order.len());

    for ch in order.chars() {
        match ch {
            'h' | 'H' => result.push(h / TAU),
            's' | 'S' => result.push(s),
            'l' | 'L' => result.push(l),
            'r' | 'R' => result.push(r),
            'g' | 'G' => result.push(g),
            'b' | 'B' => result.push(b),
            'a' | 'A' => result.push(a),
            _ => return Err(()),
        }
    }

    Ok(result)
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Error::NotFound(s) => write!(f, "Not found: '{s}'"),
            Error::BadFormat(s) => write!(f, "Bad format: '{s}'"),
            Error::BadColorSpace(s) => write!(f, "Bad color space: '{s}'"),
            Error::BadSwizzle(s) => write!(f, "Bad swizzle: '{s}'"),
            Error::InvalidIterator(s) => write!(f, "Invalid iterator: '{s}'"),
            Error::InvalidContext(s) => write!(f, "Invalid context: '{s}'"),
        }
    }
}

impl StdError for Error {}
