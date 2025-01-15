if lib == nil then
  ffi = require("ffi")

  ffi.cdef [[
    typedef struct {
      uint32_t *ptr;
      size_t len;
      bool success;
    } VecResult;
    VecResult fn_name3(const uint32_t *args, size_t len);
    void free_fn_name3(uint32_t *ptr, size_t len);
  ]]

  lib = ffi.load("E:/Documents/Training/Test/test_module/target/release/test_module.dll")
end

-- 配列を作成
local value = { 1, 3, 5 }

-- Rustに渡す値に変換
local type = "uint32_t[" .. #value .. "]"
local input = ffi.new(type, value)

-- Rust関数を呼び出す
local result = lib.fn_name3(input, #value)

if result.success then
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
  lib.free_fn_name3(result.ptr, result.len)

else
  print("Error: Failed to execute fn_name3")
end
