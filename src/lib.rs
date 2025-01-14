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

// fn_name4()で返す構造体
#[repr(C)]
pub struct StringArrayResult {
    ptr: *const *mut c_char, // 文字列ポインタの配列
    len: usize,              // 配列の長さ
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

    // 古いデータをクリア（メモリリーク対策）
    vec.clear();

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

// 引数で受け取った文字列配列を処理し、新しい文字列配列を返す関数
#[no_mangle]
pub extern "C" fn fn_name5(
    input: *const *const c_char, // 入力文字列ポインタの配列
    len: usize,                  // 配列の長さ
) -> StringArrayResult {
    // NULLチェック
    if input.is_null() {
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
        if c_str_ptr.is_null() {
            continue; // NULLポインタをスキップ
        }

        // C文字列をRust文字列に変換
        let rust_str = unsafe {
            match CStr::from_ptr(c_str_ptr).to_str() {
                Ok(s) => s,
                Err(_) => continue, // エンコーディングエラーをスキップ
            }
        };

        // 処理（例: 文字列を大文字化）
        let processed_str = rust_str.to_uppercase();

        // CStringに変換
        if let Ok(c_string) = CString::new(processed_str) {
            result_vec.push(c_string);
        }
    }

    // 結果をC互換のポインタ配列に変換
    let result_ptrs: Vec<*mut c_char> = result_vec
        .iter()
        .map(|s| s.as_ptr() as *mut c_char)
        .collect();

    // メモリリークを防ぐため、Vecの所有権を切り離す
    std::mem::forget(result_vec);

    // 結果を構造体に詰めて返す
    StringArrayResult {
        ptr: result_ptrs.as_ptr(),
        len: result_ptrs.len(),
    }
}

// ↑で作成した値をメモリから解放(メモリリーク対策、共有変数を使用しない場合)
#[no_mangle]
pub extern "C" fn free_string_array(result: StringArrayResult) {
    // 安全にポインタ配列をスライスとして扱う
    if result.ptr.is_null() {
        return;
    }
    let slice = unsafe { std::slice::from_raw_parts(result.ptr, result.len) };

    // 各ポインタをCString::from_rawで解放
    for &ptr in slice.iter() {
        if !ptr.is_null() {
            unsafe {
                let _ = CString::from_raw(ptr as *mut c_char); // メモリを解放
            }
        }
    }

    // 配列自体を解放
    unsafe {
        let _ = Vec::from_raw_parts(result.ptr as *mut *mut c_char, result.len, result.len);
    }
}
