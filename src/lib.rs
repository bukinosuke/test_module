use std::sync::{Mutex, OnceLock};

// グローバル変数の初期化
static GLOBAL_VEC: OnceLock<Mutex<Vec<u32>>> = OnceLock::new();

// VecResult構造体の定義
#[repr(C)]
pub struct VecResult {
    ptr: *mut u32, // データのポインタ
    len: usize,    // データの長さ
    success: u8,   // エラー
}

// 配列を受け取り、グローバル変数に追加して返す関数
#[no_mangle]
pub extern "C" fn fn_name3(args: *const u32, len: usize) -> VecResult {
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

// メモリを解放する関数
#[no_mangle]
pub extern "C" fn free_fn_name3(ptr: *mut u32, len: usize) {
    if !ptr.is_null() && len > 0 {
        unsafe {
            let _ = Box::from_raw(std::slice::from_raw_parts_mut(ptr, len));
        }
    }
}
