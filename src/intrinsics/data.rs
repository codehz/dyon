use std::fs::File;
use std::io::Read;

use read_token::{NumberSettings, ReadToken};

use super::io::io_error;

use Variable;

/// Loads data from a file.
pub fn load_file(file: &str) -> Result<Variable, String> {
    let mut data_file = try!(File::open(file).map_err(|err| io_error("open", file, &err)));
    let mut d = String::new();
    try!(data_file.read_to_string(&mut d).map_err(|err| io_error("read", file, &err)));
    load_data(&d)
}

/// Loads data from text.
pub fn load_data(data: &str) -> Result<Variable, String> {
    let mut read = ReadToken::new(data, 0);
    expr(&mut read)
}

static NUMBER_SETTINGS: NumberSettings = NumberSettings {
    allow_underscore: true,
};

const SEPS: &'static str = &"(){}[],.:;\n\"\\";

fn expr(read: &mut ReadToken) -> Result<Variable, String> {
    use std::sync::Arc;

    if let Some(range) = read.tag("{") {
        // Object.
        *read = read.consume(range.length);
        return object(read);
    }
    if let Some(range) = read.tag("[") {
        // Array.
        *read = read.consume(range.length);
        return array(read);
    }
    if let Some(range) = read.tag("(") {
        // Vec4.
        *read = read.consume(range.length);
        return vec4(read);
    }
    if let Some(range) = read.tag("#") {
        use read_color::rgb_maybe_a;

        // Color.
        *read = read.consume(range.length);
        let (range, _) = read.until_any_or_whitespace(SEPS);
        let val = read.raw_string(range.length);
        if let Some((rgb, a)) = rgb_maybe_a(&mut val.chars()) {
            let v = [rgb[0] as f32 / 255.0, rgb[1] as f32 / 255.0, rgb[2] as f32 / 255.0,
                     a.unwrap_or(255) as f32 / 255.0];
            return Ok(Variable::Vec4(v));
        } else {
            return Err("Expected hex color".into());
        }
    }
    if let Some(range) = read.tag("link") {
        // Link.
        *read = read.consume(range.length);
        return link(read);
    }
    // Text.
    if let Some(range) = read.string() {
        match read.parse_string(range.length) {
            Ok(s) => {
                *read = read.consume(range.length);
                return Ok(Variable::Text(Arc::new(s)));
            }
            Err(err_range) => return Err(format!("{}", err_range.data)),
        }
    }
    // Number.
    if let Some(range) = read.number(&NUMBER_SETTINGS) {
        match read.parse_number(&NUMBER_SETTINGS, range.length) {
            Ok(val) => {
                *read = read.consume(range.length);
                return Ok(Variable::f64(val));
            }
            Err(err) => return Err(format!("{}", err)),
        }
    }
    // Boolean.
    if let Some(range) = read.tag("false") {
        *read = read.consume(range.length);
        return Ok(Variable::bool(false));
    }
    if let Some(range) = read.tag("true") {
        *read = read.consume(range.length);
        return Ok(Variable::bool(true));
    }
    Err("Not implemented".into())
}

fn object(read: &mut ReadToken) -> Result<Variable, String> {
    use std::sync::Arc;
    use std::collections::HashMap;

    let mut res: HashMap<Arc<String>, Variable> = HashMap::new();
    let mut was_comma = false;
    loop {
        opt_w(read);

        if let Some(range) = read.tag("}") {
            *read = read.consume(range.length);
            break;
        }

        if res.len() > 0 && !was_comma {
            return Err("Expected `,`".into());
        }

        let (range, _) = read.until_any_or_whitespace(SEPS);
        let key: Arc<String>;
        if range.length == 0 {
            return Err("Expected key".into());
        } else {
            key = Arc::new(read.raw_string(range.length));
            *read = read.consume(range.length);
        };

        opt_w(read);

        if let Some(range) = read.tag(":") {
            *read = read.consume(range.length);
        } else {
            return Err("Expected `:`".into());
        }

        opt_w(read);

        res.insert(key, try!(expr(read)));

        was_comma = comma(read);
    }
    Ok(Variable::Object(Arc::new(res)))
}

fn array(read: &mut ReadToken) -> Result<Variable, String> {
    use std::sync::Arc;

    let mut res = vec![];
    let mut was_comma = false;
    loop {
        opt_w(read);

        if let Some(range) = read.tag("]") {
            *read = read.consume(range.length);
            break;
        }

        if res.len() > 0 && !was_comma {
            return Err("Expected `,`".into());
        }

        res.push(try!(expr(read)));
        was_comma = comma(read);
    }
    Ok(Variable::Array(Arc::new(res)))
}

fn link(read: &mut ReadToken) -> Result<Variable, String> {
    use Link;

    opt_w(read);

    if let Some(range) = read.tag("{") {
        *read = read.consume(range.length);
    } else {
        return Err("Expected `{`".into());
    }

    let mut link = Link::new();

    opt_w(read);

    loop {
        opt_w(read);

        if let Some(range) = read.tag("}") {
            *read = read.consume(range.length);
            break;
        }

        match link.push(&try!(expr(read))) {
            Ok(()) => {}
            Err(err) => return Err(err),
        };
    }
    Ok(Variable::Link(Box::new(link)))
}

fn vec4(read: &mut ReadToken) -> Result<Variable, String> {
    let x = if let Some(range) = read.number(&NUMBER_SETTINGS) {
        match read.parse_number(&NUMBER_SETTINGS, range.length) {
            Ok(x) => {
                *read = read.consume(range.length);
                x
            }
            Err(err) => return Err(format!("{}", err)),
        }
    } else {
        return Err("Expected x component".into());
    };
    comma(read);
    let y = if let Some(range) = read.number(&NUMBER_SETTINGS) {
        match read.parse_number(&NUMBER_SETTINGS, range.length) {
            Ok(y) => {
                *read = read.consume(range.length);
                y
            }
            Err(err) => return Err(format!("{}", err)),
        }
    } else {
        return Err("Expected y component".into());
    };
    let (z, w) = if comma(read) {
        if let Some(range) = read.number(&NUMBER_SETTINGS) {
            match read.parse_number(&NUMBER_SETTINGS, range.length) {
                Ok(z) => {
                    *read = read.consume(range.length);
                    comma(read);
                    if let Some(range) = read.number(&NUMBER_SETTINGS) {
                        match read.parse_number(&NUMBER_SETTINGS, range.length) {
                            Ok(w) => (z, w),
                            Err(err) => return Err(format!("{}", err)),
                        }
                    } else { (z, 0.0) }
                }
                Err(err) => return Err(format!("{}", err)),
            }
        } else { (0.0, 0.0) }
    } else { (0.0, 0.0) };
    opt_w(read);
    if let Some(range) = read.tag(")") {
        *read = read.consume(range.length);
    }
    Ok(Variable::Vec4([x as f32, y as f32, z as f32, w as f32]))
}

/// Reads optional whitespace including comments.
fn opt_w(read: &mut ReadToken) {
    let range = read.whitespace();
    if range.length > 0 {
        *read = read.consume(range.length);
    }
}

/// Reads comma.
fn comma(read: &mut ReadToken) -> bool {
    let mut res = false;
    opt_w(read);
    if let Some(range) = read.tag(",") {
        *read = read.consume(range.length);
        res = true;
    }
    opt_w(read);
    res
}
