if lib == nil then
  ffi = require("ffi")

  -- Rustで作成した関数を定義
  ffi.cdef [[
    typedef struct {
      const char** ptr;
      size_t len;
    } StringArrayResult;
    typedef struct {
      uint32_t *ptr;
      size_t len;
      uint8_t success;
    } VecResult;
    int fn_name1(int a, int b);
    uint32_t fn_name2(const uint32_t* args, size_t len);
    char* fn_name3(const char* str);
    StringArrayResult fn_name4(
    const char** input,
    size_t len
    );
    VecResult fn_name5(const uint32_t *args, size_t len);
    StringArrayResult fn_name6(const char* str);
    void free_string(char* ptr);
    void free_string_array(StringArrayResult result);
    void free_vec_u32(uint32_t *ptr, size_t len);
  ]]

  -- DLLの読み込み
  lib = ffi.load("E:/Documents/Training/Test/test_module/target/release/test_module.dll")
end


-- モジュール関数の呼び出し
local fn_name1 = lib.fn_name1(1, 2)
print(fn_name1)


-- モジュール関数に配列を渡す
local values2 = ffi.new("uint32_t[5]", { 10, 20, 30, 40, 50 })
local fn_name2 = lib.fn_name2(values2, 5)
print(fn_name2)


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


-- モジュール関数に配列の渡し、グローバル変数の操作をする
local value = { 1, 3, 5 }
-- Rustに渡す値に変換
local type = "uint32_t[" .. #value .. "]"
local input = ffi.new(type, value)
-- Rust関数を呼び出す
local result = lib.fn_name5(input, #value)
-- 成否判定
if result.success == 0 then
  -- 返ってきた値をテーブル化
  local result_table = {}
  for i = 0, tonumber(result.len) - 1 do
    table.insert(result_table, result.ptr[i])
  end
  -- テーブルを表示
  for i, value in ipairs(result_table) do
    print(value)
  end
  -- メモリを解放
  lib.free_vec_u32(result.ptr, result.len)
else
  -- エラー表示
  if result.success == 1 then
    print("Error: 引数が不正です。")
  elseif result.success == 2 then
    print("Error: GLOBAL_VECのロックに失敗しました。")
  end
end


-- モジュール関数にPSDのパスを渡し、レイヤー名のリストを受け取る
local path = "D:/Downloads/春日部つむぎ立ち絵_公式_v2.0.psd"
local result = lib.fn_name6(path)
-- 成否判定
if result.ptr ~= nil and result.len > 0 then
  -- 結果をLuaのテーブルに変換
  local layer_list = {}
  for i = 0, tonumber(result.len) - 1 do
    table.insert(layer_list, ffi.string(result.ptr[i]))
  end
  -- コンソールに出力
  for i, value in ipairs(layer_list) do
    print(i .. " / " .. value)
  end
  -- メモリ解放
  lib.free_string_array(result)
else
  print("PSDファイルの読み込みに失敗しました。(" .. path .. ")")
end
