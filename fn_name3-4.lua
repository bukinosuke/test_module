if lib == nil then
  ffi = require("ffi")

  -- Rustで作成した関数を定義
  -- 構造体の定義、関数の定義、ポインタ解放関数の定義
  ffi.cdef [[
    typedef struct {
      const char** ptr;
      size_t len;
    } StringArrayResult;

    char* fn_name3(const char* str);

    StringArrayResult fn_name4(
    const char** input,
    size_t len
    );

    void free_string(char* ptr);
    void free_string_array(StringArrayResult result);

  ]]

  -- DLLの読み込み
  lib = ffi.load("E:/Documents/Training/Test/test_module/target/release/test_module.dll")
end

-- モジュール関数に文字列を渡し、文字列を受け取る
local str = "知ってる？"
local fn_name3 = lib.fn_name3(str)
-- ポインタから文字列を取得
local string = ffi.string(fn_name3)
print(string)
-- fn_name3で作成した文字列のメモリをモジュール内から解放
lib.free_string(fn_name3)

-- モジュール関数に文字列配列を渡し、文字列配列を受け取る
local input = ffi.new("const char*[3]", { "hello", "unchi", "world" })
local fn_name4 = lib.fn_name4(input, 3)
-- 結果をLuaのテーブルに変換
local array = {}
for i = 0, tonumber(fn_name4.len) - 1 do
  table.insert(array, ffi.string(fn_name4.ptr[i]))
end
-- 出力を表示
for i, value in ipairs(array) do
  print(value)
end
-- fn_name4で作成した文字列配列のメモリをモジュール内から解放
lib.free_string_array(fn_name4)
