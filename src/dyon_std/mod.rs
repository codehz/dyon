#![allow(non_snake_case)]

use *;

mod io;
mod meta;
mod data;
mod lifetimechk;
mod functions;

#[cfg(not(feature = "http"))]
const HTTP_SUPPORT_DISABLED: &'static str = "Http support is disabled";

#[cfg(not(feature = "file"))]
const FILE_SUPPORT_DISABLED: &'static str = "File support is disabled";

dyon_fn!{fn x(v: Vec4) -> f64 {f64::from(v.0[0])}}
dyon_fn!{fn y(v: Vec4) -> f64 {f64::from(v.0[1])}}
dyon_fn!{fn z(v: Vec4) -> f64 {f64::from(v.0[2])}}
dyon_fn!{fn w(v: Vec4) -> f64 {f64::from(v.0[3])}}

pub(crate) fn s(rt: &mut Runtime) -> Result<(), String> {
    let ind: f64 = rt.pop().expect(TINVOTS);
    let ind = ind as usize;
    if ind >= 4 {return Err(format!("Index out of bounds `{}`", ind))};
    let v: [f32; 4] = rt.pop_vec4().expect(TINVOTS);
    rt.push(f64::from(v[ind]));
    Ok(())
}

dyon_fn!{fn det(m: Mat4) -> f64 {f64::from(vecmath::mat4_det(m.0))}}
dyon_fn!{fn inv(m: Mat4) -> Mat4 {Mat4(vecmath::mat4_inv(m.0))}}
dyon_fn!{fn mov(v: Vec4) -> Mat4 {Mat4([
    [1.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 1.0, 0.0],
    [v.0[0], v.0[1], v.0[2], 1.0],
])}}
dyon_fn!{fn rot__axis_angle(axis: Vec4, ang: f64) -> Mat4 {
    let axis = [f64::from(axis.0[0]), f64::from(axis.0[1]), f64::from(axis.0[2])];
    let cos = ang.cos();
    let sin = ang.sin();
    let inv_cos = 1.0 - cos;
    Mat4([
        [
            (cos + axis[0] * axis[0] * inv_cos) as f32,
            (axis[0] * axis[1] * inv_cos - axis[2] * sin) as f32,
            (axis[0] * axis[2] * inv_cos + axis[1] * sin) as f32,
            0.0
        ],
        [
            (axis[1] * axis[0] * inv_cos + axis[2] * sin) as f32,
            (cos + axis[1] * axis[1] * inv_cos) as f32,
            (axis[1] * axis[2] * inv_cos - axis[0] * sin) as f32,
            0.0
        ],
        [
            (axis[2] * axis[0] * inv_cos - axis[1] * sin) as f32,
            (axis[2] * axis[1] * inv_cos + axis[0] * sin) as f32,
            (cos + axis[2] * axis[2] * inv_cos) as f32,
            0.0
        ],
        [0.0,0.0,0.0,1.0]
    ])
}}
dyon_fn!{fn ortho__pos_right_up_forward(pos: Vec4, right: Vec4, up: Vec4, forward: Vec4) -> Mat4 {
    use vecmath::vec4_dot as dot;
    Mat4([
        [right.0[0], up.0[0], forward.0[0], 0.0],
        [right.0[1], up.0[1], forward.0[1], 0.0],
        [right.0[2], up.0[2], forward.0[2], 0.0],
        [-dot(right.0, pos.0), -dot(up.0, pos.0), -dot(forward.0, pos.0), 1.0],
    ])
}}
dyon_fn!{fn proj__fov_near_far_ar(fov: f64, near: f64, far: f64, ar: f64) -> Mat4 {
    let f = 1.0 / (fov * ::std::f64::consts::PI).tan();
    Mat4([
        [(f/ar) as f32, 0.0, 0.0, 0.0],
        [0.0, f as f32, 0.0, 0.0],
        [0.0, 0.0, ((far + near) / (near - far)) as f32, -1.0],
        [0.0, 0.0, ((2.0 * far * near) / (near - far)) as f32, 0.0],
    ])
}}
dyon_fn!{fn mvp__model_view_projection(model: Mat4, view: Mat4, proj: Mat4) -> Mat4 {
    use vecmath::col_mat4_mul as mul;
    Mat4(mul(mul(proj.0, view.0), model.0))
}}
dyon_fn!{fn scale(v: Vec4) -> Mat4 {Mat4([
    [v.0[0], 0.0, 0.0, 0.0],
    [0.0, v.0[1], 0.0, 0.0],
    [0.0, 0.0, v.0[2], 0.0],
    [0.0, 0.0, 0.0, 1.0],
])}}

dyon_fn!{fn rx(m: Mat4) -> Vec4 {Vec4([m.0[0][0], m.0[1][0], m.0[2][0], m.0[3][0]])}}
dyon_fn!{fn ry(m: Mat4) -> Vec4 {Vec4([m.0[0][1], m.0[1][1], m.0[2][1], m.0[3][1]])}}
dyon_fn!{fn rz(m: Mat4) -> Vec4 {Vec4([m.0[0][2], m.0[1][2], m.0[2][2], m.0[3][2]])}}
dyon_fn!{fn rw(m: Mat4) -> Vec4 {Vec4([m.0[0][3], m.0[1][3], m.0[2][3], m.0[3][3]])}}

pub(crate) fn rv(rt: &mut Runtime) -> Result<(), String> {
    let ind: f64 = rt.pop().expect(TINVOTS);
    let ind = ind as usize;
    if ind >= 4 {return Err(format!("Index out of bounds `{}`", ind))};
    let m: [[f32; 4]; 4] = rt.pop_mat4().expect(TINVOTS);
    rt.stack.push(Variable::Vec4([m[0][ind], m[1][ind], m[2][ind], m[3][ind]]));
    Ok(())
}

dyon_fn!{fn cx(m: Mat4) -> Vec4 {Vec4(m.0[0])}}
dyon_fn!{fn cy(m: Mat4) -> Vec4 {Vec4(m.0[1])}}
dyon_fn!{fn cz(m: Mat4) -> Vec4 {Vec4(m.0[2])}}
dyon_fn!{fn cw(m: Mat4) -> Vec4 {Vec4(m.0[3])}}

pub(crate) fn cv(rt: &mut Runtime) -> Result<(), String> {
    let ind: f64 = rt.pop().expect(TINVOTS);
    let ind = ind as usize;
    if ind >= 4 {return Err(format!("Index out of bounds `{}`", ind))};
    let m: [[f32; 4]; 4] = rt.pop_mat4().expect(TINVOTS);
    rt.stack.push(Variable::Vec4(m[ind]));
    Ok(())
}

pub(crate) fn clone(rt: &mut Runtime) -> Result<(), String> {
    let v = rt.stack.pop().expect(TINVOTS);
    let v = rt.resolve(&v).deep_clone(&rt.stack);
    rt.stack.push(v);
    Ok(())
}

pub(crate) fn why(rt: &mut Runtime) -> Result<(), String> {
    let v = rt.stack.pop().expect(TINVOTS);
    let v = Variable::Array(Arc::new(match rt.resolve(&v) {
        &Variable::Bool(true, Some(ref sec)) => {
            let mut sec = (**sec).clone();
            sec.reverse();
            sec
        }
        &Variable::Bool(true, None) => {
            return Err({
                rt.arg_err_index.set(Some(0));
                "This does not make sense, perhaps an array is empty?".into()
            })
        }
        &Variable::Bool(false, _) => {
            return Err({
                rt.arg_err_index.set(Some(0));
                "Must be `true` to have meaning, try add or remove `!`".into()
            })
        }
        x => return Err(rt.expected_arg(0, x, "bool"))
    }));
    rt.stack.push(v);
    Ok(())
}

pub(crate) fn _where(rt: &mut Runtime) -> Result<(), String> {
    let v = rt.stack.pop().expect(TINVOTS);
    let v = Variable::Array(Arc::new(match rt.resolve(&v) {
        &Variable::F64(val, Some(ref sec)) => {
            if val.is_nan() {
                return Err({
                    rt.arg_err_index.set(Some(0));
                    "Expected number, found `NaN`".into()
                })
            } else {
                let mut sec = (**sec).clone();
                sec.reverse();
                sec
            }
        }
        &Variable::F64(_, None) => {
            return Err({
                rt.arg_err_index.set(Some(0));
                "This does not make sense, perhaps an array is empty?".into()
            })
        }
        x => return Err(rt.expected_arg(0, x, "f64"))
    }));
    rt.stack.push(v);
    Ok(())
}

pub(crate) fn explain_why(rt: &mut Runtime) -> Result<(), String> {
    let why = rt.stack.pop().expect(TINVOTS);
    let val = rt.stack.pop().expect(TINVOTS);
    let (val, why) = match rt.resolve(&val) {
        &Variable::Bool(val, ref sec) => (val,
            match *sec {
                None => Box::new(vec![why.deep_clone(&rt.stack)]),
                Some(ref sec) => {
                    let mut sec = sec.clone();
                    sec.push(why.deep_clone(&rt.stack));
                    sec
                }
            }
        ),
        x => return Err(rt.expected_arg(0, x, "bool"))
    };
    rt.stack.push(Variable::Bool(val, Some(why)));
    Ok(())
}

pub(crate) fn explain_where(rt: &mut Runtime) -> Result<(), String> {
    let wh = rt.stack.pop().expect(TINVOTS);
    let val = rt.stack.pop().expect(TINVOTS);
    let (val, wh) = match rt.resolve(&val) {
        &Variable::F64(val, ref sec) => (val,
            match *sec {
                None => Box::new(vec![wh.deep_clone(&rt.stack)]),
                Some(ref sec) => {
                    let mut sec = sec.clone();
                    sec.push(wh.deep_clone(&rt.stack));
                    sec
                }
            }
        ),
        x => return Err(rt.expected_arg(0, x, "bool"))
    };
    rt.stack.push(Variable::F64(val, Some(wh)));
    Ok(())
}

pub(crate) fn println(rt: &mut Runtime) -> Result<(), String> {
    use write::{print_variable, EscapeString};

    let x = rt.stack.pop().expect(TINVOTS);
    print_variable(rt, &x, EscapeString::None);
    println!();
    Ok(())
}

pub(crate) fn print(rt: &mut Runtime) -> Result<(), String> {
    use write::{print_variable, EscapeString};

    let x = rt.stack.pop().expect(TINVOTS);
    print_variable(rt, &x, EscapeString::None);
    Ok(())
}

dyon_fn!{fn sqrt(a: f64) -> f64 {a.sqrt()}}
dyon_fn!{fn sin(a: f64) -> f64 {a.sin()}}
dyon_fn!{fn asin(a: f64) -> f64 {a.asin()}}
dyon_fn!{fn cos(a: f64) -> f64 {a.cos()}}
dyon_fn!{fn acos(a: f64) -> f64 {a.acos()}}
dyon_fn!{fn tan(a: f64) -> f64 {a.tan()}}
dyon_fn!{fn atan(a: f64) -> f64 {a.atan()}}
dyon_fn!{fn atan2(y: f64, x: f64) -> f64 {y.atan2(x)}}
dyon_fn!{fn exp(a: f64) -> f64 {a.exp()}}
dyon_fn!{fn ln(a: f64) -> f64 {a.ln()}}
dyon_fn!{fn log2(a: f64) -> f64 {a.log2()}}
dyon_fn!{fn log10(a: f64) -> f64 {a.log10()}}
dyon_fn!{fn round(a: f64) -> f64 {a.round()}}
dyon_fn!{fn abs(a: f64) -> f64 {a.abs()}}
dyon_fn!{fn floor(a: f64) -> f64 {a.floor()}}
dyon_fn!{fn ceil(a: f64) -> f64 {a.ceil()}}
dyon_fn!{fn sleep(v: f64) {
    use std::thread::sleep;
    use std::time::Duration;

    let secs = v as u64;
    let nanos = (v.fract() * 1.0e9) as u32;
    sleep(Duration::new(secs, nanos));
}}

pub(crate) fn head(rt: &mut Runtime) -> Result<(), String> {
    let v = rt.stack.pop().expect(TINVOTS);
    let v = Variable::Option(match rt.resolve(&v) {
        &Variable::Link(ref link) => link.head(),
        x => return Err(rt.expected_arg(0, x, "link"))
    });
    rt.stack.push(v);
    Ok(())
}

pub(crate) fn tip(rt: &mut Runtime) -> Result<(), String> {
    let v = rt.stack.pop().expect(TINVOTS);
    let v = Variable::Option(match rt.resolve(&v) {
        &Variable::Link(ref link) => link.tip(),
        x => return Err(rt.expected_arg(0, x, "link"))
    });
    rt.stack.push(v);
    Ok(())
}

pub(crate) fn tail(rt: &mut Runtime) -> Result<(), String> {
    let v = rt.stack.pop().expect(TINVOTS);
    let v = Variable::Link(Box::new(match rt.resolve(&v) {
        &Variable::Link(ref link) => link.tail(),
        x => return Err(rt.expected_arg(0, x, "link"))
    }));
    rt.stack.push(v);
    Ok(())
}

pub(crate) fn neck(rt: &mut Runtime) -> Result<(), String> {
    let v = rt.stack.pop().expect(TINVOTS);
    let v = Variable::Link(Box::new(match rt.resolve(&v) {
        &Variable::Link(ref link) => link.neck(),
        x => return Err(rt.expected_arg(0, x, "link"))
    }));
    rt.stack.push(v);
    Ok(())
}

pub(crate) fn is_empty(rt: &mut Runtime) -> Result<(), String> {
    let v = rt.stack.pop().expect(TINVOTS);
    let v = Variable::bool(match rt.resolve(&v) {
        &Variable::Link(ref link) => link.is_empty(),
        x => return Err(rt.expected_arg(0, x, "link"))
    });
    rt.stack.push(v);
    Ok(())
}

pub(crate) fn random(rt: &mut Runtime) -> Result<(), String> {
    use rand::Rng;

    let v: f64 = rt.rng.gen();
    rt.push(v);
    Ok(())
}

dyon_fn!{fn tau() -> f64 {6.283_185_307_179_586}}

pub(crate) fn len(rt: &mut Runtime) -> Result<(), String> {
    let v = rt.stack.pop().expect(TINVOTS);
    let v = {
        let arr = match rt.resolve(&v) {
            &Variable::Array(ref arr) => arr,
            x => return Err(rt.expected_arg(0, x, "array"))
        };
        Variable::f64(arr.len() as f64)
    };
    rt.stack.push(v);
    Ok(())
}

pub(crate) fn push_ref(rt: &mut Runtime) -> Result<(), String> {
    let item = rt.stack.pop().expect(TINVOTS);
    let v = rt.stack.pop().expect(TINVOTS);

    if let Variable::Ref(ind) = v {
        let ok = if let Variable::Array(ref mut arr) = rt.stack[ind] {
            Arc::make_mut(arr).push(item);
            true
        } else {
            false
        };
        if !ok {
            return Err({
                rt.arg_err_index.set(Some(0));
                "Expected reference to array".into()
            })
        }
    } else {
        return Err({
            rt.arg_err_index.set(Some(0));
            "Expected reference to array".into()
        })
    }
    Ok(())
}

pub(crate) fn insert_ref(rt: &mut Runtime) -> Result<(), String> {
    let item = rt.stack.pop().expect(TINVOTS);
    let index = rt.stack.pop().expect(TINVOTS);
    let index = match rt.resolve(&index) {
        &Variable::F64(index, _) => index,
        x => return Err(rt.expected_arg(1, x, "number"))
    };
    let v = rt.stack.pop().expect(TINVOTS);

    if let Variable::Ref(ind) = v {
        if let Variable::Array(ref arr) = rt.stack[ind] {
            let index = index as usize;
            if index > arr.len() {
                return Err("Index out of bounds".into())
            }
        }
        let ok = if let Variable::Array(ref mut arr) = rt.stack[ind] {
            Arc::make_mut(arr).insert(index as usize, item);
            true
        } else {
            false
        };
        if !ok {
            return Err({
                rt.arg_err_index.set(Some(0));
                "Expected reference to array".into()
            })
        }
    } else {
        return Err({
            rt.arg_err_index.set(Some(0));
            "Expected reference to array".into()
        })
    }
    Ok(())
}

pub(crate) fn push(rt: &mut Runtime) -> Result<(), String> {
    let item = rt.stack.pop().expect(TINVOTS);
    let item = rt.resolve(&item).deep_clone(&rt.stack);
    let v = rt.stack.pop().expect(TINVOTS);

    if let Variable::Ref(ind) = v {
        let ok = if let Variable::Array(ref mut arr) = rt.stack[ind] {
            Arc::make_mut(arr).push(item);
            true
        } else {
            false
        };
        if !ok {
            return Err({
                rt.arg_err_index.set(Some(0));
                "Expected reference to array".into()
            })
        }
    } else {
        return Err({
            rt.arg_err_index.set(Some(0));
            "Expected reference to array".into()
        })
    }
    Ok(())
}

pub(crate) fn insert(rt: &mut Runtime) -> Result<(), String> {
    let item = rt.stack.pop().expect(TINVOTS);
    let item = rt.resolve(&item).deep_clone(&rt.stack);
    let index = rt.stack.pop().expect(TINVOTS);
    let index = match rt.resolve(&index) {
        &Variable::F64(index, _) => index,
        x => return Err(rt.expected_arg(1, x, "number"))
    };
    let v = rt.stack.pop().expect(TINVOTS);

    if let Variable::Ref(ind) = v {
        if let Variable::Array(ref arr) = rt.stack[ind] {
            let index = index as usize;
            if index > arr.len() {
                return Err("Index out of bounds".into())
            }
        }
        let ok = if let Variable::Array(ref mut arr) = rt.stack[ind] {
            Arc::make_mut(arr).insert(index as usize, item);
            true
        } else {
            false
        };
        if !ok {
            return Err({
                rt.arg_err_index.set(Some(0));
                "Expected reference to array".into()
            })
        }
    } else {
        return Err({
            rt.arg_err_index.set(Some(0));
            "Expected reference to array".into()
        })
    }
    Ok(())
}

pub(crate) fn pop(rt: &mut Runtime) -> Result<(), String> {
    let arr = rt.stack.pop().expect(TINVOTS);
    let mut v: Option<Variable> = None;
    if let Variable::Ref(ind) = arr {
        let ok = if let Variable::Array(ref mut arr) = rt.stack[ind] {
            v = Arc::make_mut(arr).pop();
            true
        } else {
            false
        };
        if !ok {
            return Err({
                rt.arg_err_index.set(Some(0));
                "Expected reference to array".into()
            })
        }
    } else {
        return Err({
            rt.arg_err_index.set(Some(0));
            "Expected reference to array".into()
        })
    }
    let v = match v {
        None => return Err({
            rt.arg_err_index.set(Some(0));
            "Expected non-empty array".into()
        }),
        Some(val) => val
    };
    rt.stack.push(v);
    Ok(())
}

pub(crate) fn remove(rt: &mut Runtime) -> Result<(), String> {
    let index = rt.stack.pop().expect(TINVOTS);
    let index = match rt.resolve(&index) {
        &Variable::F64(index, _) => index,
        x => return Err(rt.expected_arg(1, x, "number"))
    };
    let arr = rt.stack.pop().expect(TINVOTS);
    if let Variable::Ref(ind) = arr {
        if let Variable::Array(ref arr) = rt.stack[ind] {
            let index = index as usize;
            if index >= arr.len() {
                return Err("Index out of bounds".into())
            }
        }
        if let Variable::Array(ref mut arr) = rt.stack[ind] {
            let v = Arc::make_mut(arr).remove(index as usize);
            rt.stack.push(v);
            return Ok(());
        };
        return Err({
            rt.arg_err_index.set(Some(0));
            "Expected reference to array".into()
        })
    } else {
        return Err({
            rt.arg_err_index.set(Some(0));
            "Expected reference to array".into()
        })
    }
}

pub(crate) fn reverse(rt: &mut Runtime) -> Result<(), String> {
    let v = rt.stack.pop().expect(TINVOTS);
    if let Variable::Ref(ind) = v {
        let ok = if let Variable::Array(ref mut arr) = rt.stack[ind] {
            Arc::make_mut(arr).reverse();
            true
        } else {
            false
        };
        if !ok {
            return Err({
                rt.arg_err_index.set(Some(0));
                "Expected reference to array".into()
            })
        }
    } else {
        return Err({
            rt.arg_err_index.set(Some(0));
            "Expected reference to array".into()
        })
    }
    Ok(())
}

pub(crate) fn clear(rt: &mut Runtime) -> Result<(), String> {
    let v = rt.stack.pop().expect(TINVOTS);
    if let Variable::Ref(ind) = v {
        let ok = if let Variable::Array(ref mut arr) = rt.stack[ind] {
            Arc::make_mut(arr).clear();
            true
        } else {
            false
        };
        if !ok {
            return Err({
                rt.arg_err_index.set(Some(0));
                "Expected reference to array".into()
            })
        }
    } else {
        return Err({
            rt.arg_err_index.set(Some(0));
            "Expected reference to array".into()
        })
    }
    Ok(())
}

pub(crate) fn swap(rt: &mut Runtime) -> Result<(), String> {
    let j = rt.stack.pop().expect(TINVOTS);
    let i = rt.stack.pop().expect(TINVOTS);
    let j = match rt.resolve(&j) {
        &Variable::F64(val, _) => val,
        x => return Err(rt.expected_arg(2, x, "number"))
    };
    let i = match rt.resolve(&i) {
        &Variable::F64(val, _) => val,
        x => return Err(rt.expected_arg(1, x, "number"))
    };
    let v = rt.stack.pop().expect(TINVOTS);
    if let Variable::Ref(ind) = v {
        let ok = if let Variable::Array(ref mut arr) = rt.stack[ind] {
            Arc::make_mut(arr).swap(i as usize, j as usize);
            true
        } else {
            false
        };
        if !ok {
            return Err({
                rt.arg_err_index.set(Some(0));
                "Expected reference to array".into()
            })
        }
    } else {
        return Err({
            rt.arg_err_index.set(Some(0));
            "Expected reference to array".into()
        })
    }
    Ok(())
}

pub(crate) fn read_line(rt: &mut Runtime) -> Result<(), String> {
    use std::io::{self, Write};
    use std::error::Error;

    let mut input = String::new();
    io::stdout().flush().unwrap();
    let error = match io::stdin().read_line(&mut input) {
        Ok(_) => None,
        Err(error) => Some(error)
    };
    rt.push(if let Some(error) = error {
        return Err(error.description().into())
    } else {
        Variable::Text(Arc::new(input))
    });
    Ok(())
}

pub(crate) fn read_number(rt: &mut Runtime) -> Result<(), String> {
    use std::io::{self, Write};
    use std::error::Error;

    let err: Arc<String> = rt.pop().expect(TINVOTS);
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut input = String::new();
    let rv = loop {
        input.clear();
        stdout.flush().unwrap();
        match stdin.read_line(&mut input) {
            Ok(_) => {}
            Err(error) => return Err(error.description().into()),
        };
        match input.trim().parse::<f64>() {
            Ok(v) => break v,
            Err(_) => println!("{}", err),
        }
    };
    rt.push(rv);
    Ok(())
}

dyon_fn!{fn parse_number(text: Arc<String>) -> Option<f64> {text.trim().parse::<f64>().ok()}}
dyon_fn!{fn trim(v: Arc<String>) -> Arc<String> {Arc::new(v.trim().into())}}
dyon_fn!{fn trim_left(v: Arc<String>) -> Arc<String> {Arc::new(v.trim_start().into())}}
dyon_fn!{fn trim_right(v: Arc<String>) -> Arc<String> {Arc::new(v.trim_end().into())}}

pub(crate) fn _str(rt: &mut Runtime) -> Result<(), String> {
    use write::{write_variable, EscapeString};

    let v = rt.stack.pop().expect(TINVOTS);
    let mut buf: Vec<u8> = vec![];
    write_variable(&mut buf, rt, rt.resolve(&v), EscapeString::None, 0).unwrap();
    rt.push(String::from_utf8(buf).unwrap());
    Ok(())
}

pub(crate) fn json_string(rt: &mut Runtime) -> Result<(), String> {
    use write::{write_variable, EscapeString};

    let v = rt.stack.pop().expect(TINVOTS);
    let mut buf: Vec<u8> = vec![];
    write_variable(&mut buf, rt, rt.resolve(&v), EscapeString::Json, 0).unwrap();
    rt.stack.push(Variable::Text(Arc::new(String::from_utf8(buf).unwrap())));
    Ok(())
}

dyon_fn!{fn str__color(v: Vec4) -> Arc<String> {
    let v = v.0;
    let mut buf: Vec<u8> = vec![];
    let clamp = |x| {
        if x < 0.0 { 0.0 } else if x > 1.0 { 1.0 } else { x }
    };
    let r = (clamp(v[0]) * 255.0) as usize;
    let g = (clamp(v[1]) * 255.0) as usize;
    let b = (clamp(v[2]) * 255.0) as usize;
    let a = (clamp(v[3]) * 255.0) as usize;
    let map = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
               'a', 'b', 'c', 'd', 'e', 'f'];
    let (r1, r2) = (r >> 4, r & 0xf);
    let (g1, g2) = (g >> 4, g & 0xf);
    let (b1, b2) = (b >> 4, b & 0xf);
    let (a1, a2) = (a >> 4, a & 0xf);
    buf.push(b'#');
    buf.push(map[r1] as u8); buf.push(map[r2] as u8);
    buf.push(map[g1] as u8); buf.push(map[g2] as u8);
    buf.push(map[b1] as u8); buf.push(map[b2] as u8);
    if a != 255 {
        buf.push(map[a1] as u8); buf.push(map[a2] as u8);
    }
    Arc::new(String::from_utf8(buf).unwrap())
}}

dyon_fn!{fn srgb_to_linear__color(v: Vec4) -> Vec4 {
    let v = v.0;
    let to_linear = |f: f32| {
        if f <= 0.04045 {
            f / 12.92
        } else {
            ((f + 0.055) / 1.055).powf(2.4)
        }
    };
    Vec4([to_linear(v[0]), to_linear(v[1]), to_linear(v[2]), v[3]])
}}

dyon_fn!{fn linear_to_srgb__color(v: Vec4) -> Vec4 {
    let v = v.0;
    let to_srgb = |f: f32| {
        if f <= 0.003_130_8 {
            f * 12.92
        } else {
            1.055 * f.powf(1.0 / 2.4) - 0.055
        }
    };
    Vec4([to_srgb(v[0]), to_srgb(v[1]), to_srgb(v[2]), v[3]])
}}

pub(crate) fn _typeof(rt: &mut Runtime) -> Result<(), String> {
    use crate::runtime::*;
    use crate::Variable::*;

    let v = rt.stack.pop().expect(TINVOTS);
    let t = Variable::Text(match *rt.resolve(&v) {
        Text(_) => TEXT_TYPE.clone(),
        F64(_, _) => F64_TYPE.clone(),
        Vec4(_) => VEC4_TYPE.clone(),
        Mat4(_) => MAT4_TYPE.clone(),
        Return => RETURN_TYPE.clone(),
        Bool(_, _) => BOOL_TYPE.clone(),
        Object(_) => OBJECT_TYPE.clone(),
        Array(_) => ARRAY_TYPE.clone(),
        Link(_) => LINK_TYPE.clone(),
        Ref(_) => REF_TYPE.clone(),
        UnsafeRef(_) => UNSAFE_REF_TYPE.clone(),
        RustObject(_) => RUST_OBJECT_TYPE.clone(),
        Option(_) => OPTION_TYPE.clone(),
        Result(_) => RESULT_TYPE.clone(),
        Thread(_) => THREAD_TYPE.clone(),
        Closure(_, _) => CLOSURE_TYPE.clone(),
        In(_) => IN_TYPE.clone(),
    });
    rt.stack.push(t);
    Ok(())
}

pub(crate) fn debug(rt: &mut Runtime) -> Result<(), String> {
    println!("Stack {:#?}", rt.stack);
    println!("Locals {:#?}", rt.local_stack);
    println!("Currents {:#?}", rt.current_stack);
    Ok(())
}

pub(crate) fn backtrace(rt: &mut Runtime) -> Result<(), String> {
    println!("{:#?}", rt.call_stack);
    Ok(())
}

pub(crate) fn load(rt: &mut Runtime) -> Result<(), String> {
    use load;

    let v = rt.stack.pop().expect(TINVOTS);
    let v = match rt.resolve(&v) {
        &Variable::Text(ref text) => {
            let mut m = Module::new_intrinsics(rt.module.intrinsics.clone());
            for f in &rt.module.ext_prelude {
                m.add(f.name.clone(), f.f, f.p.clone());
            }
            if let Err(err) = load(text, &mut m) {
                Variable::Result(Err(Box::new(Error {
                    message: Variable::Text(Arc::new(format!("When attempting to load module:\n{}", err))),
                    trace: vec![]
                })))
            } else {
                Variable::Result(Ok(Box::new(
                    Variable::RustObject(Arc::new(Mutex::new(Arc::new(m)))))))
            }
        }
        x => {
            return Err(rt.expected_arg(0, x, "string"));
        }
    };
    rt.stack.push(v);
    Ok(())
}

pub(crate) fn load__source_imports(rt: &mut Runtime) -> Result<(), String> {
    use load;

    let modules = rt.stack.pop().expect(TINVOTS);
    let source = rt.stack.pop().expect(TINVOTS);
    let mut new_module = Module::new_intrinsics(rt.module.intrinsics.clone());
    for f in &rt.module.ext_prelude {
        new_module.add(f.name.clone(), f.f, f.p.clone());
    }
    let x = rt.resolve(&modules);
    match x {
        &Variable::Array(ref array) => {
            for it in &**array {
                match rt.resolve(it) {
                    &Variable::RustObject(ref obj) => {
                        match obj.lock().unwrap().downcast_ref::<Arc<Module>>() {
                            Some(m) => {
                                // Add external functions from imports.
                                for f in &m.ext_prelude {
                                    let has_external = new_module.ext_prelude.iter()
                                        .any(|a| a.name == f.name);
                                    if !has_external {
                                        new_module.add(f.name.clone(), f.f, f.p.clone());
                                    }
                                }
                                // Register loaded functions from imports.
                                for f in &m.functions {
                                    new_module.register(f.clone())
                                }
                            }
                            None => return Err(rt.expected_arg(1, x, "[Module]"))
                        }
                    }
                    x => return Err(rt.expected_arg(1, x, "[Module]"))
                }
            }
        }
        x => return Err(rt.expected_arg(1, x, "[Module]"))
    }
    let v = match rt.resolve(&source) {
        &Variable::Text(ref text) => {
            if let Err(err) = load(text, &mut new_module) {
                Variable::Result(Err(Box::new(Error {
                    message: Variable::Text(Arc::new(
                        format!("When attempting to load module:\n{}", err))),
                    trace: vec![]
                })))
            } else {
                Variable::Result(Ok(Box::new(
                    Variable::RustObject(Arc::new(
                        Mutex::new(Arc::new(new_module)))))))
            }
        }
        x => return Err(rt.expected_arg(0, x, "str"))
    };
    rt.stack.push(v);
    Ok(())
}

pub(crate) fn module__in_string_imports(rt: &mut Runtime) -> Result<(), String> {
    let modules = rt.stack.pop().expect(TINVOTS);
    let source = rt.stack.pop().expect(TINVOTS);
    let source = match rt.resolve(&source) {
        &Variable::Text(ref t) => t.clone(),
        x => return Err(rt.expected_arg(1, x, "str"))
    };
    let name = rt.stack.pop().expect(TINVOTS);
    let name = match rt.resolve(&name) {
        &Variable::Text(ref t) => t.clone(),
        x => return Err(rt.expected_arg(0, x, "str"))
    };
    let mut new_module = Module::new_intrinsics(rt.module.intrinsics.clone());
    for f in &rt.module.ext_prelude {
        new_module.add(f.name.clone(), f.f, f.p.clone());
    }
    let x = rt.resolve(&modules);
    match x {
        &Variable::Array(ref array) => {
            for it in &**array {
                match rt.resolve(it) {
                    &Variable::RustObject(ref obj) => {
                        match obj.lock().unwrap().downcast_ref::<Arc<Module>>() {
                            Some(m) => {
                                // Add external functions from imports.
                                for f in &m.ext_prelude {
                                    let has_external = new_module.ext_prelude.iter()
                                        .any(|a| a.name == f.name);
                                    if !has_external {
                                        new_module.add(f.name.clone(), f.f, f.p.clone());
                                    }
                                }
                                // Register loaded functions from imports.
                                for f in &m.functions {
                                    new_module.register(f.clone())
                                }
                            }
                            None => return Err(rt.expected_arg(2, x, "[Module]"))
                        }
                    }
                    x => return Err(rt.expected_arg(2, x, "[Module]"))
                }
            }
        }
        x => return Err(rt.expected_arg(2, x, "[Module]"))
    }
    let v = if let Err(err) = load_str(&name, source, &mut new_module) {
            Variable::Result(Err(Box::new(Error {
                message: Variable::Text(Arc::new(
                    format!("When attempting to load module:\n{}", err))),
                trace: vec![]
            })))
        } else {
            Variable::Result(Ok(Box::new(
                Variable::RustObject(Arc::new(
                    Mutex::new(Arc::new(new_module)))))))
        };
    rt.stack.push(v);
    Ok(())
}

pub(crate) fn _call(rt: &mut Runtime) -> Result<(), String> {
    // Use the source from calling function.
    let source = rt.module.functions[rt.call_stack.last().unwrap().index].source.clone();
    let args = rt.stack.pop().expect(TINVOTS);
    let fn_name = rt.stack.pop().expect(TINVOTS);
    let call_module = rt.stack.pop().expect(TINVOTS);
    let fn_name = match rt.resolve(&fn_name) {
        &Variable::Text(ref text) => text.clone(),
        x => return Err(rt.expected_arg(1, x, "text"))
    };
    let args = match rt.resolve(&args) {
        &Variable::Array(ref arr) => arr.clone(),
        x => return Err(rt.expected_arg(2, x, "array"))
    };
    let x = rt.resolve(&call_module);
    let obj = match x {
        &Variable::RustObject(ref obj) => obj.clone(),
        x => return Err(rt.expected_arg(0, x, "Module"))
    };

    match obj.lock().unwrap()
        .downcast_ref::<Arc<Module>>() {
        Some(m) => {
            use std::cell::Cell;

            let f_index = m.find_function(&fn_name, 0);
            match f_index {
                FnIndex::Loaded(f_index) => {
                    let f = &m.functions[f_index as usize];
                    if f.args.len() != args.len() {
                        return Err({
                            rt.arg_err_index.set(Some(2));
                            format!(
                                "Expected `{}` arguments, found `{}`",
                                f.args.len(), args.len()
                            )
                        })
                    }
                    lifetimechk::check(f, &args).map_err(|err| {
                        rt.arg_err_index.set(Some(2));
                        err
                    })?;
                }
                FnIndex::Intrinsic(_) | FnIndex::None |
                FnIndex::ExternalVoid(_) | FnIndex::ExternalReturn(_) =>
                    return Err(format!("Could not find function `{}`", fn_name))
            }
            // Use empty range instead of `call.source_range` (from when it was intrinsic).
            let call_range = Range::empty(0);
            let call = ast::Call {
                alias: None,
                name: fn_name.clone(),
                f_index: Cell::new(f_index),
                args: args.iter().map(|arg|
                    ast::Expression::Variable(Box::new((
                        call_range, arg.clone())))).collect(),
                custom_source: Some(source),
                source_range: call_range,
            };

            rt.call(&call, &m)?;
        }
        None => return Err(rt.expected_arg(0, x, "Module"))
    }

    Ok(())
}

pub(crate) fn call_ret(rt: &mut Runtime) -> Result<(), String> {
    // Use the source from calling function.
    let source = rt.module.functions[rt.call_stack.last().unwrap().index].source.clone();
    let args = rt.stack.pop().expect(TINVOTS);
    let fn_name = rt.stack.pop().expect(TINVOTS);
    let call_module = rt.stack.pop().expect(TINVOTS);
    let args = match rt.resolve(&args) {
        &Variable::Array(ref arr) => arr.clone(),
        x => return Err(rt.expected_arg(2, x, "array"))
    };
    let fn_name = match rt.resolve(&fn_name) {
        &Variable::Text(ref text) => text.clone(),
        x => return Err(rt.expected_arg(1, x, "text"))
    };
    let x = rt.resolve(&call_module);
    let obj = match x {
        &Variable::RustObject(ref obj) => obj.clone(),
        x => return Err(rt.expected_arg(0, x, "Module"))
    };

    let v = match obj.lock().unwrap()
        .downcast_ref::<Arc<Module>>() {
        Some(m) => {
            use std::cell::Cell;

            let f_index = m.find_function(&fn_name, 0);
            match f_index {
                FnIndex::Loaded(f_index) => {
                    let f = &m.functions[f_index as usize];
                    if f.args.len() != args.len() {
                        return Err({
                            rt.arg_err_index.set(Some(2));
                            format!(
                                "Expected `{}` arguments, found `{}`",
                                f.args.len(), args.len()
                            )
                        })
                    }
                    lifetimechk::check(f, &args).map_err(|err| {
                        rt.arg_err_index.set(Some(2));
                        err
                    })?;
                }
                FnIndex::Intrinsic(_) | FnIndex::None |
                FnIndex::ExternalVoid(_) | FnIndex::ExternalReturn(_) =>
                    return Err(format!("Could not find function `{}`", fn_name))
            }
            // Use empty range instead of `call.source_range` (from when it was intrinsic).
            let call_range = Range::empty(0);
            let call = ast::Call {
                alias: None,
                name: fn_name.clone(),
                f_index: Cell::new(f_index),
                args: args.iter().map(|arg|
                    ast::Expression::Variable(Box::new((
                        call_range, arg.clone())))).collect(),
                custom_source: Some(source),
                source_range: call_range,
            };

            if let Some(v) = rt.call(&call, &m)?.0 {v} else {
                return Err(format!("Expected some return value `{}`", fn_name))
            }
        }
        None => return Err(rt.expected_arg(0, x, "Module"))
    };

    rt.stack.push(v);
    Ok(())
}

pub(crate) fn functions(rt: &mut Runtime) -> Result<(), String> {
    // List available functions in scope.
    let v = Variable::Array(Arc::new(functions::list_functions(&rt.module)));
    rt.stack.push(v);
    Ok(())
}

pub(crate) fn functions__module(rt: &mut Runtime) -> Result<(), String> {
    // List available functions in scope.
    let m = rt.stack.pop().expect(TINVOTS);
    let x = rt.resolve(&m);
    let m = match x {
        &Variable::RustObject(ref obj) => obj.clone(),
        x => return Err(rt.expected_arg(0, x, "Module"))
    };

    let functions = match m.lock().unwrap()
        .downcast_ref::<Arc<Module>>() {
        Some(m) => functions::list_functions(m),
        None => return Err(rt.expected_arg(0, x, "Module"))
    };

    let v = Variable::Array(Arc::new(functions));
    rt.push(v);
    Ok(())
}

dyon_fn!{fn none() -> Option<Variable> {None}}

pub(crate) fn some(rt: &mut Runtime) -> Result<(), String> {
    let v = rt.stack.pop().expect(TINVOTS);
    let v = Variable::Option(Some(Box::new(
        rt.resolve(&v).deep_clone(&rt.stack)
    )));
    rt.stack.push(v);
    Ok(())
}

pub(crate) fn ok(rt: &mut Runtime) -> Result<(), String> {
    let v = rt.stack.pop().expect(TINVOTS);
    let v = Variable::Result(Ok(Box::new(
        rt.resolve(&v).deep_clone(&rt.stack)
    )));
    rt.stack.push(v);
    Ok(())
}

pub(crate) fn err(rt: &mut Runtime) -> Result<(), String> {
    let v = rt.stack.pop().expect(TINVOTS);
    let v = Variable::Result(Err(Box::new(
        Error {
            message: rt.resolve(&v).deep_clone(&rt.stack),
            trace: vec![]
        })));
    rt.stack.push(v);
    Ok(())
}

pub(crate) fn is_err(rt: &mut Runtime) -> Result<(), String> {
    let v = rt.stack.pop().expect(TINVOTS);
    let v = match rt.resolve(&v) {
        &Variable::Result(Err(_)) => Variable::bool(true),
        &Variable::Result(Ok(_)) => Variable::bool(false),
        x => return Err(rt.expected_arg(0, x, "result"))
    };
    rt.stack.push(v);
    Ok(())
}

pub(crate) fn is_ok(rt: &mut Runtime) -> Result<(), String> {
    let v = rt.stack.pop().expect(TINVOTS);
    let v = match rt.resolve(&v) {
        &Variable::Result(Err(_)) => Variable::bool(false),
        &Variable::Result(Ok(_)) => Variable::bool(true),
        x => return Err(rt.expected_arg(0, x, "result"))
    };
    rt.stack.push(v);
    Ok(())
}

pub(crate) fn min(rt: &mut Runtime) -> Result<(), String> {
    let v = rt.stack.pop().expect(TINVOTS);
    let v = match rt.resolve(&v) {
        &Variable::Array(ref arr) => {
            let mut min: f64 = ::std::f64::NAN;
            for v in &**arr {
                if let Variable::F64(val, _) = *rt.resolve(v) {
                    if val < min || min.is_nan() { min = val }
                }
            }
            min
        }
        x => return Err(rt.expected_arg(0, x, "array"))
    };
    rt.stack.push(Variable::f64(v));
    Ok(())
}

pub(crate) fn max(rt: &mut Runtime) -> Result<(), String> {
    let v = rt.stack.pop().expect(TINVOTS);
    let v = match rt.resolve(&v) {
        &Variable::Array(ref arr) => {
            let mut max: f64 = ::std::f64::NAN;
            for v in &**arr {
                if let Variable::F64(val, _) = *rt.resolve(v) {
                    if val > max || max.is_nan() { max = val }
                }
            }
            max
        }
        x => return Err(rt.expected_arg(0, x, "array"))
    };
    rt.stack.push(Variable::f64(v));
    Ok(())
}

pub(crate) fn unwrap(rt: &mut Runtime) -> Result<(), String> {
    use write::{write_variable, EscapeString};

    // Return value does not depend on lifetime of argument since
    // `ok(x)` and `some(x)` perform a deep clone.
    let v = rt.stack.pop().expect(TINVOTS);
    let v = match rt.resolve(&v) {
        &Variable::Option(Some(ref v)) => (**v).clone(),
        &Variable::Option(None) => {
            return Err({
                rt.arg_err_index.set(Some(0));
                "Expected `some(_)`".into()
            })
        }
        &Variable::Result(Ok(ref ok)) => (**ok).clone(),
        &Variable::Result(Err(ref err)) => {
            use std::str::from_utf8;

            // Print out error message.
            let mut w: Vec<u8> = vec![];
            w.extend_from_slice(rt.stack_trace().as_bytes());
            w.extend_from_slice("\n".as_bytes());
            write_variable(&mut w, rt, &err.message,
                           EscapeString::None, 0).unwrap();
            for t in &err.trace {
                w.extend_from_slice("\n".as_bytes());
                w.extend_from_slice(t.as_bytes());
            }
            return Err({
                rt.arg_err_index.set(Some(0));
                from_utf8(&w).unwrap().into()
            })
        }
        x => return Err(rt.expected_arg(0, x, "some(_) or ok(_)"))
    };
    rt.stack.push(v);
    Ok(())
}

pub(crate) fn unwrap_or(rt: &mut Runtime) -> Result<(), String> {
    // Return value does not depend on lifetime of argument since
    // `ok(x)` and `some(x)` perform a deep clone.
    let def = rt.stack.pop().expect(TINVOTS);
    let v = rt.stack.pop().expect(TINVOTS);
    let v = match rt.resolve(&v) {
        &Variable::Option(Some(ref v)) => (**v).clone(),
        &Variable::Result(Ok(ref ok)) => (**ok).clone(),
        &Variable::Option(None) |
        &Variable::Result(Err(_)) => rt.resolve(&def).clone(),
        x => return Err(rt.expected_arg(0, x, "some(_) or ok(_)"))
    };
    rt.stack.push(v);
    Ok(())
}

pub(crate) fn unwrap_err(rt: &mut Runtime) -> Result<(), String> {
    let v = rt.stack.pop().expect(TINVOTS);
    let v = match rt.resolve(&v) {
        &Variable::Result(Err(ref err)) => err.message.clone(),
        x => return Err(rt.expected_arg(0, x, "err(_)"))
    };
    rt.stack.push(v);
    Ok(())
}

dyon_fn!{fn dir__angle(val: f64) -> Vec4 {Vec4([val.cos() as f32, val.sin() as f32, 0.0, 0.0])}}

dyon_fn!{fn load__meta_file(meta: Arc<String>, file: Arc<String>) -> Variable {
    let res = meta::load_meta_file(&**meta, &**file);
    Variable::Result(match res {
        Ok(res) => Ok(Box::new(Variable::Array(Arc::new(res)))),
        Err(err) => Err(Box::new(Error {
            message: Variable::Text(Arc::new(err)),
            trace: vec![]
        }))
    })
}}

dyon_fn!{fn load__meta_url(meta: Arc<String>, url: Arc<String>) -> Variable {
    let res = meta::load_meta_url(&**meta, &**url);
    Variable::Result(match res {
        Ok(res) => Ok(Box::new(Variable::Array(Arc::new(res)))),
        Err(err) => Err(Box::new(Error {
            message: Variable::Text(Arc::new(err)),
            trace: vec![]
        }))
    })
}}

dyon_fn!{fn syntax__in_string(name: Arc<String>, text: Arc<String>) -> Variable {
    use piston_meta::syntax_errstr;

    let res = syntax_errstr(&text).map_err(|err|
        format!("When parsing meta syntax in `{}`:\n{}", name, err));
    Variable::Result(match res {
        Ok(res) => Ok(Box::new(Variable::RustObject(Arc::new(Mutex::new(Arc::new(res)))))),
        Err(err) => Err(Box::new(Error {
            message: Variable::Text(Arc::new(err)),
            trace: vec![]
        }))
    })
}}

pub(crate) fn meta__syntax_in_string(rt: &mut Runtime) -> Result<(), String> {
    use piston_meta::Syntax;

    let text = rt.stack.pop().expect(TINVOTS);
    let text = match rt.resolve(&text) {
        &Variable::Text(ref t) => t.clone(),
        x => return Err(rt.expected_arg(2, x, "str"))
    };
    let name = rt.stack.pop().expect(TINVOTS);
    let name = match rt.resolve(&name) {
        &Variable::Text(ref t) => t.clone(),
        x => return Err(rt.expected_arg(1, x, "str"))
    };
    let syntax_var = rt.stack.pop().expect(TINVOTS);
    let syntax_var = rt.resolve(&syntax_var);
    let syntax = match syntax_var {
        &Variable::RustObject(ref obj) => obj.clone(),
        x => return Err(rt.expected_arg(0, x, "Syntax"))
    };
    let res = meta::parse_syntax_data(match syntax.lock().unwrap()
        .downcast_ref::<Arc<Syntax>>() {
        Some(s) => s,
        None => return Err(rt.expected_arg(0, syntax_var, "Syntax"))
    }, &name, &text);
    let v = Variable::Result(match res {
        Ok(res) => Ok(Box::new(Variable::Array(Arc::new(res)))),
        Err(err) => Err(Box::new(Error {
            message: Variable::Text(Arc::new(err)),
            trace: vec![]
        }))
    });
    rt.stack.push(v);
    Ok(())
}

dyon_fn!{fn download__url_file(url: Arc<String>, file: Arc<String>) -> Variable {
    let res = meta::download_url_to_file(&**url, &**file);
    Variable::Result(match res {
        Ok(res) => Ok(Box::new(Variable::Text(Arc::new(res)))),
        Err(err) => Err(Box::new(Error {
            message: Variable::Text(Arc::new(err)),
            trace: vec![]
        }))
    })
}}

#[cfg(feature = "file")]
dyon_fn!{fn save__string_file(text: Arc<String>, file: Arc<String>) -> Variable {
    use std::fs::File;
    use std::error::Error as StdError;
    use std::io::Write;

    Variable::Result(match File::create(&**file) {
        Ok(mut f) => {
            match f.write_all(text.as_bytes()) {
                Ok(_) => Ok(Box::new(Variable::Text(file))),
                Err(err) => Err(Box::new(Error {
                    message: Variable::Text(Arc::new(err.description().into())),
                    trace: vec![]
                }))
            }
        }
        Err(err) => Err(Box::new(Error {
            message: Variable::Text(Arc::new(err.description().into())),
            trace: vec![]
        }))
    })
}}

#[cfg(not(feature = "file"))]
pub(crate) fn save__string_file(_: &mut Runtime) -> Result<(), String> {
    Err(FILE_SUPPORT_DISABLED.into())
}

#[cfg(feature = "file")]
dyon_fn!{fn load_string__file(file: Arc<String>) -> Variable {
    use std::fs::File;
    use std::io::Read;
    use std::error::Error as StdError;

    Variable::Result(match File::open(&**file) {
        Ok(mut f) => {
            let mut s = String::new();
            match f.read_to_string(&mut s) {
                Ok(_) => {
                    Ok(Box::new(Variable::Text(Arc::new(s))))
                }
                Err(err) => {
                    Err(Box::new(Error {
                        message: Variable::Text(Arc::new(err.description().into())),
                        trace: vec![]
                    }))
                }
            }
        }
        Err(err) => Err(Box::new(Error {
            message: Variable::Text(Arc::new(err.description().into())),
            trace: vec![]
        }))
    })
}}

#[cfg(not(feature = "file"))]
pub(crate) fn load_string__file(_: &mut Runtime) -> Result<(), String> {
    Err(FILE_SUPPORT_DISABLED.into())
}

dyon_fn!{fn load_string__url(url: Arc<String>) -> Variable {
    Variable::Result(match meta::load_text_file_from_url(&**url) {
        Ok(s) => {
            Ok(Box::new(Variable::Text(Arc::new(s))))
        }
        Err(err) => {
            Err(Box::new(Error {
                message: Variable::Text(Arc::new(err)),
                trace: vec![]
            }))
        }
    })
}}

pub(crate) fn join__thread(rt: &mut Runtime) -> Result<(), String> {
    let thread = rt.stack.pop().expect(TINVOTS);
    let handle_res = Thread::invalidate_handle(rt, thread);
    let v = Variable::Result({
        match handle_res {
            Ok(handle) => {
                match handle.join() {
                    Ok(res) => match res {
                        Ok(res) => Ok(Box::new(res)),
                        Err(err) => Err(Box::new(Error {
                            message: Variable::Text(Arc::new(err)),
                            trace: vec![]
                        }))
                    },
                    Err(_err) => Err(Box::new(Error {
                        message: Variable::Text(Arc::new(
                            "Thread did not exit successfully".into())),
                        trace: vec![]
                    }))
                }
            }
            Err(err) => {
                Err(Box::new(Error {
                    message: Variable::Text(Arc::new(err)),
                    trace: vec![]
                }))
            }
        }
    });
    rt.push(v);
    Ok(())
}

dyon_fn!{fn load_data__file(file: Arc<String>) -> Variable {
    use Error;

    let res = match data::load_file(&file) {
        Ok(data) => Ok(Box::new(data)),
        Err(err) => Err(Box::new(Error {
            message: Variable::Text(Arc::new(format!(
                        "Error loading data from file `{}`:\n{}",
                        file, err))),
            trace: vec![]
        }))
    };
    Variable::Result(res)
}}

dyon_fn!{fn load_data__string(text: Arc<String>) -> Variable {
    use Error;

    let res = match data::load_data(&text) {
        Ok(data) => Ok(Box::new(data)),
        Err(err) => Err(Box::new(Error {
            message: Variable::Text(Arc::new(format!(
                        "Error loading data from string `{}`:\n{}",
                        text, err))),
            trace: vec![]
        }))
    };
    Variable::Result(res)
}}

pub(crate) fn args_os(rt: &mut Runtime) -> Result<(), String> {
    let mut arr: Vec<Variable> = vec![];
    for arg in ::std::env::args_os() {
        if let Ok(t) = arg.into_string() {
            arr.push(Variable::Text(Arc::new(t)))
        } else {
            return Err("Invalid unicode in os argument".into());
        }
    }
    rt.stack.push(Variable::Array(Arc::new(arr)));
    Ok(())
}

#[cfg(feature = "file")]
pub(crate) fn save__data_file(rt: &mut Runtime) -> Result<(), String> {
    use std::error::Error;
    use std::fs::File;
    use std::io::BufWriter;
    use write::{write_variable, EscapeString};

    let file = rt.stack.pop().expect(TINVOTS);
    let file = match rt.resolve(&file) {
        &Variable::Text(ref t) => t.clone(),
        x => return Err(rt.expected_arg(1, x, "str"))
    };
    let data = rt.stack.pop().expect(TINVOTS);

    let mut f = match File::create(&**file) {
        Ok(f) => BufWriter::new(f),
        Err(err) => {
            return Err({
                rt.arg_err_index.set(Some(0));
                format!("Error when creating file `{}`:\n{}",
                 file, err.description())
            })
        }
    };
    let res = match write_variable(&mut f, rt, &data, EscapeString::Json, 0) {
        Ok(()) => Ok(Box::new(Variable::Text(file.clone()))),
        Err(err) => {
            Err(Box::new(::Error {
                message: Variable::Text(Arc::new(format!(
                            "Error when writing to file `{}`:\n{}",
                            file, err.description()))),
                trace: vec![]
            }))
        }
    };
    rt.stack.push(Variable::Result(res));
    Ok(())
}

#[cfg(not(feature = "file"))]
pub(crate) fn save__data_file(_: &mut Runtime) -> Result<(), String> {
    Err(FILE_SUPPORT_DISABLED.into())
}

pub(crate) fn json_from_meta_data(rt: &mut Runtime) -> Result<(), String> {
    use std::error::Error;

    let meta_data = rt.stack.pop().expect(TINVOTS);
    let json = match rt.resolve(&meta_data) {
        &Variable::Array(ref arr) => {
            meta::json_from_meta_data(arr).map_err(|err| {
                format!("{}\nError when generating JSON:\n{}",
                        rt.stack_trace(),
                        err.description())
            })?
        }
        x => return Err(rt.expected_arg(0, x, "array"))
    };
    rt.stack.push(Variable::Text(Arc::new(json)));
    Ok(())
}

pub(crate) fn errstr__string_start_len_msg(rt: &mut Runtime) -> Result<(), String> {
    use piston_meta::ParseErrorHandler;

    let msg = rt.stack.pop().expect(TINVOTS);
    let msg = match rt.resolve(&msg) {
        &Variable::Text(ref t) => t.clone(),
        x => return Err(rt.expected_arg(3, x, "str"))
    };
    let len = rt.stack.pop().expect(TINVOTS);
    let len = match rt.resolve(&len) {
        &Variable::F64(v, _) => v as usize,
        x => return Err(rt.expected_arg(2, x, "f64"))
    };
    let start = rt.stack.pop().expect(TINVOTS);
    let start = match rt.resolve(&start) {
        &Variable::F64(v, _) => v as usize,
        x => return Err(rt.expected_arg(1, x, "f64"))
    };
    let source = rt.stack.pop().expect(TINVOTS);
    let source = match rt.resolve(&source) {
        &Variable::Text(ref t) => t.clone(),
        x => return Err(rt.expected_arg(0, x, "str"))
    };

    let mut buf: Vec<u8> = vec![];
    ParseErrorHandler::new(&source)
        .write_msg(&mut buf, Range::new(start, len), &msg)
        .unwrap();
    rt.stack.push(Variable::Text(Arc::new(String::from_utf8(buf).unwrap())));
    Ok(())
}

pub(crate) fn has(rt: &mut Runtime) -> Result<(), String> {
    let key = rt.stack.pop().expect(TINVOTS);
    let key = match rt.resolve(&key) {
        &Variable::Text(ref t) => t.clone(),
        x => return Err(rt.expected_arg(1, x, "str"))
    };
    let obj = rt.stack.pop().expect(TINVOTS);
    let res = match rt.resolve(&obj) {
        &Variable::Object(ref obj) => obj.contains_key(&key),
        x => return Err(rt.expected_arg(0, x, "object"))
    };
    rt.stack.push(Variable::bool(res));
    Ok(())
}

pub(crate) fn keys(rt: &mut Runtime) -> Result<(), String> {
    let obj = rt.stack.pop().expect(TINVOTS);
    let res = Variable::Array(Arc::new(match rt.resolve(&obj) {
        &Variable::Object(ref obj) => {
            obj.keys().map(|k| Variable::Text(k.clone())).collect()
        }
        x => return Err(rt.expected_arg(0, x, "object"))
    }));
    rt.stack.push(res);
    Ok(())
}

pub(crate) fn chars(rt: &mut Runtime) -> Result<(), String> {
    let t = rt.stack.pop().expect(TINVOTS);
    let t = match rt.resolve(&t) {
        &Variable::Text(ref t) => t.clone(),
        x => return Err(rt.expected_arg(0, x, "str"))
    };
    let res = t.chars()
        .map(|ch| {
            let mut s = String::new();
            s.push(ch);
            Variable::Text(Arc::new(s))
        })
        .collect::<Vec<_>>();
    rt.stack.push(Variable::Array(Arc::new(res)));
    Ok(())
}

dyon_fn!{fn now() -> f64 {
    use std::time::{SystemTime, UNIX_EPOCH};

    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(val) => val.as_secs() as f64 +
                   f64::from(val.subsec_nanos()) / 1.0e9,
        Err(err) => -{
            let val = err.duration();
            val.as_secs() as f64 +
            f64::from(val.subsec_nanos()) / 1.0e9
        }
    }
}}

dyon_fn!{fn is_nan(v: f64) -> bool {v.is_nan()}}

pub(crate) fn wait_next(rt: &mut Runtime) -> Result<(), String> {
    use std::error::Error;

    let v = rt.stack.pop().expect(TINVOTS);
    let v = match rt.resolve(&v) {
        &Variable::In(ref mutex) => {
            match mutex.lock() {
                Ok(x) => match x.recv() {
                    Ok(x) => Variable::Option(Some(Box::new(x))),
                    Err(_) => Variable::Option(None),
                },
                Err(err) =>
                    return Err(format!("Can not lock In mutex:\n{}", err.description()))
            }
        }
        x => return Err(rt.expected_arg(0, x, "in"))
    };
    rt.stack.push(v);
    Ok(())
}

pub(crate) fn next(rt: &mut Runtime) -> Result<(), String> {
    use std::error::Error;

    let v = rt.stack.pop().expect(TINVOTS);
    let v = match rt.resolve(&v) {
        &Variable::In(ref mutex) => {
            match mutex.lock() {
                Ok(x) => match x.try_recv() {
                    Ok(x) => Variable::Option(Some(Box::new(x))),
                    Err(_) => Variable::Option(None),
                },
                Err(err) =>
                    return Err(format!("Can not lock In mutex:\n{}", err.description()))
            }
        }
        x => return Err(rt.expected_arg(0, x, "in"))
    };
    rt.stack.push(v);
    Ok(())
}
