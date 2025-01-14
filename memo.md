### ffiを利用したdllの作成
- 場合によっては`mlua`クレートの利用を検討

#### プロジェクトの作成
```shell
$ cargo new [] --lib
```

#### Cargo.tomlに追記
```rust
[lib]
crate-type = ["cdylib"]
```

#### 本文コード
```rust
// no_mangleは関数名をリネームされることを防ぐ
#[no_mangle]
pub extern "C" fn fn_name1(a: i32, b: i32) -> i32 {
    a + b
}

// 配列受け取り用
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
```

#### luaからの呼び出し
```lua
if lib == nil then
    ffi = require("ffi")

    -- Rustで作成した関数を定義
    ffi.cdef [[
    int fn_name1(int a, int b);
    uint32_t fn_name2(const uint32_t* args, size_t len);
    ]]

    -- DLLの読み込み
    lib = ffi.load("E:/Documents/Training/Test/test_module/target/release/test_module.dll")

    print(lib)
end

-- モジュール関数の呼び出し
local fn_name1 = lib.fn_name1(1, 2)
print(fn_name1)

-- モジュール関数に配列を渡す
local values = ffi.new("uint32_t[5]", {10, 20, 30, 40, 50})
local fn_name2 = lib.fn_name2(values, 5)
print(fn_name2)

```