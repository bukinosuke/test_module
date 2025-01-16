use psd::Psd;
use std::ffi::{CStr, CString};
use std::fs;
use std::os::raw::c_char;
use std::sync::{Mutex, OnceLock};

// グローバル変数の初期化
static GLOBAL_VEC: OnceLock<Mutex<Vec<u32>>> = OnceLock::new();

// 文字列配列の構造体
#[repr(C)]
pub struct StringArrayResult {
    ptr: *const *mut c_char, // 文字列ポインタの配列
    len: usize,              // 配列の長さ
}

// エラー番号付きVec<u32>のコード
#[repr(C)]
pub struct VecResult {
    ptr: *mut u32, // データのポインタ
    len: usize,    // データの長さ
    success: u8,   // エラー
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

// 配列を受け取り、グローバル変数に追加して返す関数
#[no_mangle]
pub extern "C" fn fn_name5(args: *const u32, len: usize) -> VecResult {
    // 引数が不正だった場合
    if args.is_null() || len == 0 {
        return VecResult {
            ptr: std::ptr::null_mut(),
            len: 0,
            success: 1,
        };
    }

    // 引数のスライスを作成
    let slice = unsafe { std::slice::from_raw_parts(args, len) };

    // グローバル変数を取得(もしくは初期化)
    let global_vec = GLOBAL_VEC.get_or_init(|| Mutex::new(Vec::new()));

    // グローバルを変数ロック
    let mut vec = match global_vec.lock() {
        Ok(v) => v,
        Err(_) => {
            // ロック失敗時
            return VecResult {
                ptr: std::ptr::null_mut(),
                len: 0,
                success: 2,
            };
        }
    };

    // スライスをグローバル変数に追加
    vec.extend_from_slice(slice);

    // 新しい値を作成
    let new_value: Vec<u32> = vec.iter().map(|&u| u * 5).collect();

    // グローバル変数のロック解除
    drop(vec);

    // データをボックス化
    let boxed_slice = new_value.into_boxed_slice();
    let ptr = boxed_slice.as_ptr() as *mut u32;
    let len = boxed_slice.len();

    // データをRustの管理外に
    std::mem::forget(boxed_slice);

    // 構造体を返す
    VecResult {
        ptr,
        len,
        success: 0,
    }
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

    // 文字列構造体を返す
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

// Vec<u32>を解放(メモリ解放)
#[no_mangle]
pub extern "C" fn free_vec_u32(ptr: *mut u32, len: usize) {
    if !ptr.is_null() && len > 0 {
        unsafe {
            let _ = Box::from_raw(std::slice::from_raw_parts_mut(ptr, len));
        }
    }
}
