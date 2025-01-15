// 文字列や文字列配列の受け渡しをする関数

use std::ffi::{CStr, CString}; // CStrとCStringをインポート
use std::os::raw::c_char;

// fn_name4()で返す構造体
#[repr(C)]
pub struct StringArrayResult {
    ptr: *const *mut c_char, // 文字列ポインタの配列
    len: usize,              // 配列の長さ
}

// 文字列を受け取り、文字列を返す
#[no_mangle]
pub extern "C" fn fn_name3(value: *const c_char) -> *mut c_char {
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

// 引数で受け取った文字列配列を処理し、新しい文字列配列を返す関数
#[no_mangle]
pub extern "C" fn fn_name4(
    input: *const *const c_char, // 入力文字列ポインタの配列
    len: usize,                  // 配列の長さ
) -> StringArrayResult {
    // nullチェック
    if input.is_null() || len == 0 {
        return StringArrayResult {
            ptr: std::ptr::null(),
            len: 0,
        };
    }

    // Rust側でスライスとして扱う
    let input_slice = unsafe { std::slice::from_raw_parts(input, len) };

    // 結果用のVecを作成
    let mut result_vec = Vec::new();

    for &c_str_ptr in input_slice.iter() {
        // nullポインタをスキップ
        if c_str_ptr.is_null() {
            continue;
        }

        // C文字列をRust文字列に変換
        let rust_str = unsafe {
            CStr::from_ptr(c_str_ptr)
                .to_str()
                .unwrap_or("Invalid UTF-8")
        };

        // 処理（例: 文字列を大文字化）
        let processed_str = rust_str.to_uppercase();

        // CStringに変換
        if let Ok(c_string) = CString::new(processed_str) {
            result_vec.push(c_string.into_raw());
        }
    }

    // データをボックス化
    let result_ptrs = result_vec.into_boxed_slice();
    let len = result_ptrs.len();

    // 構造体を返す
    StringArrayResult {
        ptr: Box::into_raw(result_ptrs) as *const *mut c_char,
        len,
    }
}

// 文字列を解放(メモリ解放)
#[no_mangle]
pub extern "C" fn free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        let _ = unsafe { CString::from_raw(ptr) };
    }
}

// 文字列配列を解放(メモリ解放)
#[no_mangle]
pub extern "C" fn free_string_array(array: StringArrayResult) {
    // ポインタが不正な場合終了
    if array.ptr.is_null() || array.len == 0 {
        return;
    }

    let slice = unsafe { std::slice::from_raw_parts(array.ptr, array.len) };

    // 各ポインタを解放
    for &ptr in slice.iter() {
        if !ptr.is_null() {
            unsafe {
                let _ = CString::from_raw(ptr);
            }
        }
    }

    // 配列自体を解放
    unsafe {
        let _ = Box::from_raw(array.ptr as *mut *mut c_char);
    }
}
