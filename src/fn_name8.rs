// 共有変数のポインタを渡すコード、クローン不要

use std::sync::{Mutex, OnceLock};

#[repr(C)]
pub struct CRGBAW {
    r_ptr: *const u32,
    g_ptr: *const u32,
    b_ptr: *const u32,
    a_ptr: *const u32,
    w_ptr: *const bool,
    len: usize,
}

struct RGBAW {
    r: Box<[u32]>,
    g: Box<[u32]>,
    b: Box<[u32]>,
    a: Box<[u32]>,
    w: Box<[bool]>,
}

impl RGBAW {
    pub fn new() -> Self {
        let mut r = vec![0; 3].into_boxed_slice();
        for (i, r) in r.iter_mut().enumerate() {
            *r = i as u32;
        }

        let mut g = vec![0; 3].into_boxed_slice();
        for (i, g) in g.iter_mut().enumerate() {
            *g = (10 + i) as u32;
        }

        let mut b = vec![0; 3].into_boxed_slice();
        for (i, b) in b.iter_mut().enumerate() {
            *b = (100 + i) as u32;
        }

        let mut a = vec![0; 3].into_boxed_slice();
        for (i, a) in a.iter_mut().enumerate() {
            *a = (1000 + i) as u32;
        }

        let mut w = vec![false; 3].into_boxed_slice();
        for (i, w) in w.iter_mut().enumerate() {
            if i % 2 == 0 {
                *w = true;
            }
        }

        return RGBAW {
            r: r,
            g: g,
            b: b,
            a: a,
            w: w,
        };
    }
}

// グローバル変数の初期化
static GLOBAL_VALUE: OnceLock<Mutex<RGBAW>> = OnceLock::new();

// グローバル変数の値に+1して返す関数
#[no_mangle]
pub extern "C" fn fn_name8() -> CRGBAW {
    // グローバル変数を取得(もしくは初期化)
    let global_value = GLOBAL_VALUE.get_or_init(|| Mutex::new(RGBAW::new()));

    // グローバル変数をロック
    let mut value = global_value.lock().unwrap();

    // グローバル変数を加算
    for r in value.r.iter_mut() {
        *r += 1;
    }
    for g in value.g.iter_mut() {
        *g += 1;
    }
    for b in value.b.iter_mut() {
        *b += 1;
    }
    for a in value.a.iter_mut() {
        *a += 1;
    }
    for w in value.w.iter_mut() {
        if *w {
            *w = false;
        } else {
            *w = true;
        }
    }

    return CRGBAW {
        r_ptr: value.r.as_ptr(),
        g_ptr: value.g.as_ptr(),
        b_ptr: value.b.as_ptr(),
        a_ptr: value.a.as_ptr(),
        w_ptr: value.w.as_ptr(),
        len: value.r.len(),
    };
}
