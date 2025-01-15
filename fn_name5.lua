if lib == nil then
  ffi = require("ffi")

  -- 構造体の定義、関数の定義、ポインタ解放関数の定義
  ffi.cdef [[
    typedef struct {
      uint32_t *ptr;
      size_t len;
      uint8_t success;
    } VecResult;
    VecResult fn_name5(const uint32_t *args, size_t len);
    void free_vec_u32(uint32_t *ptr, size_t len);
  ]]

  lib = ffi.load("E:/Documents/Training/Test/test_module/target/release/test_module.dll")
end

-- 配列を作成
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
