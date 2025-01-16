// 渡されたPSDのパスから、レイヤー名一覧を返す関数

use psd::Psd;
use std::ffi::{CStr, CString};
use std::fs;
use std::os::raw::c_char;

// 返り値用の文字列配列構造体
#[repr(C)]
pub struct StringArrayResult {
    ptr: *const *mut c_char, // 文字列ポインタの配列
    len: usize,              // 配列の長さ
}

// PSDのパスを受け取り、レイヤー名を構造体にして返す
#[no_mangle]
pub extern "C" fn fn_name6(value: *const c_char) -> StringArrayResult {
    // 入力ポインタがNULLでないか確認
    if value.is_null() {
        return StringArrayResult {
            ptr: std::ptr::null(),
            len: 0,
        };
    }

    // C文字列をRustの&strに変換
    let c_str = unsafe { CStr::from_ptr(value) };
    let path = match c_str.to_str() {
        Ok(str) => str,
        Err(_) => {
            return StringArrayResult {
                ptr: std::ptr::null(),
                len: 0,
            };
        }
    };

    // パスが有効かを判定
    if path.is_empty() || !std::path::Path::new(path).exists() {
        return StringArrayResult {
            ptr: std::ptr::null(),
            len: 0,
        };
    }

    // PSDファイルを取得
    let psd_read = match fs::read(path) {
        Ok(p) => p,
        Err(_) => {
            return StringArrayResult {
                ptr: std::ptr::null(),
                len: 0,
            };
        }
    };

    // PSD型に変換
    let psd_bytes = match Psd::from_bytes(&psd_read) {
        Ok(psd) => psd,
        Err(_) => {
            return StringArrayResult {
                ptr: std::ptr::null(),
                len: 0,
            };
        }
    };

    // レイヤー名をリスト化
    let mut layer_list = Vec::new();

    // レイヤー名をCStringに変換して追加
    for layer in psd_bytes.layers().iter() {
        if let Ok(c_string) = CString::new(layer.name()) {
            layer_list.push(c_string.into_raw());
        }
    }

    // データをボックス化
    let result_ptrs = layer_list.into_boxed_slice();
    let len = result_ptrs.len();

    // 構造体を返す
    StringArrayResult {
        ptr: Box::into_raw(result_ptrs) as *const *mut c_char,
        len,
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
