// ※dll→dllになるためかクラッシュするので利用不可

// Goを呼び出し、値を受け取る
#[no_mangle]
pub extern "C" fn fn_name7() -> *mut c_char {
    // DLLのロード(※ここでクラッシュする)
    let lib =
        unsafe { Library::new("E:/Documents/Training/Test/test_module/Go/test_module_go.dll") };

    let lib = match lib {
        Ok(l) => l,
        Err(_) => return CString::new("libでエラー").unwrap().into_raw(),
    };

    // goFunction関数のシンボルを取得
    let go_function = unsafe { lib.get(b"goFunction\0") };

    let go_function: Symbol<unsafe extern "C" fn(*const c_char) -> *mut c_char> = match go_function
    {
        Ok(f) => f,
        Err(_) => return CString::new("go_functionでエラー").unwrap().into_raw(),
    };

    // goFree関数のシンボルを取得
    let go_free = unsafe { lib.get(b"goFree\0") };

    let go_free: Symbol<unsafe extern "C" fn(*mut c_char)> = match go_free {
        Ok(f) => f,
        Err(_) => return CString::new("go_freeでエラー").unwrap().into_raw(),
    };

    // Goに渡す文字列
    let str = "ウンチーコングって・・・";

    // ↑をCに変換
    let c_str = match CString::new(str) {
        Ok(c_string) => c_string.into_raw(), // メモリの所有権をC側に移動
        Err(_) => std::ptr::null_mut(),
    };

    // Go関数を呼び出し
    let go_str = unsafe { go_function(c_str) };

    // C文字列をRustの&strに変換
    let c_str = unsafe { CStr::from_ptr(go_str) };
    let str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => "",
    };

    let string = format!("{}\nそんなー(´・ω・｀)", str);

    // Goで受け取った値を解放
    unsafe {
        go_free(go_str);
    };

    // CStringに変換し、ポインタを返す
    match CString::new(string) {
        Ok(c_string) => c_string.into_raw(), // メモリの所有権をC側に移動
        Err(_) => std::ptr::null_mut(),
    }
}
