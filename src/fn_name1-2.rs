// 基本的な値の受け渡しをする関数

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