use std::ffi::{CStr, CString}; // CStrとCStringをインポート
use std::os::raw::c_char;
use std::sync::{Mutex, OnceLock}; // c_char型をインポート

// 返り値のVecを共有変数化(再利用しないとメモリリークの原因になる)
static GLOBAL_VEC: OnceLock<Mutex<Vec<u32>>> = OnceLock::new();

// fn_name3()で返す構造体
#[repr(C)]
pub struct VecResult {
    ptr: *const u32, // データのポインタ
    len: usize,      // データの長さ
}

// 受け取った値を足して返す関数
#[no_mangle]
pub extern "C" fn fn_name1(a: i32, b: i32) -> i32 {
    a + b
}

// 配列を受け取り合算して返す関数
#[no_mangle]
pub extern "C" fn fn_name2(args: *const u32, len: usize) -> u32 {
    // ポインタと長さからスライスを作成
    let slice = unsafe { std::slice::from_raw_parts(args, len) };

    // スライスを合算
    let mut value = 0;
    for arg in slice.iter() {
        value += arg;
    }
    return value;
}

// 配列を受け取り、配列の値を倍にして追加して返す関数
#[no_mangle]
pub extern "C" fn fn_name3(args: *const u32, len: usize) -> VecResult {
    // ポインタと長さからスライスを作成
    let slice = unsafe { std::slice::from_raw_parts(args, len) };

    // 共有変数の参照を取得
    let global_vec = GLOBAL_VEC.get_or_init(|| Mutex::new(Vec::new()));
    // 共有変数をロック
    let mut vec = global_vec.lock().unwrap();

    // 配列の値を倍にし、配列の末尾に追加
    let mut value = Vec::new();
    for u in vec.iter() {
        value.push(*u);
    }
    for arg in slice.iter() {
        value.push(*arg * 2);
    }

    // ↑で作成した配列を所有権ごと共有変数に渡す
    *vec = value;

    // 構造体にしてデータを返す
    return VecResult {
        ptr: vec.as_ptr(), // 生ポインタを取得
        len: vec.len(),    // 長さを取得
    };
}

// 文字列を受け取り、文字列を返す
#[no_mangle]
pub extern "C" fn fn_name4(value: *const c_char) -> *mut c_char {
    // 入力ポインタがNULLでないか確認
    if value.is_null() {
        return std::ptr::null_mut();
    }

    // C文字列をRustの&strに変換
    let c_str = unsafe { CStr::from_ptr(value) };
    let value_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    // 挨拶文字列を生成
    let str = format!("ウンチーコングって、{}", value_str);

    // CStringに変換し、ポインタを返す
    match CString::new(str) {
        Ok(c_string) => c_string.into_raw(), // メモリの所有権をC側に移動
        Err(_) => std::ptr::null_mut(),
    }
}

// ↑で作成した値をメモリから解放(メモリリーク対策、共有変数を使用しない場合)
#[no_mangle]
pub extern "C" fn free_string(ptr: *mut c_char) {
    if ptr.is_null() {
        return;
    }
    // メモリを解放
    let _ = unsafe { CString::from_raw(ptr) };
}
