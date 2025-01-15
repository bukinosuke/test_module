if lib == nil then
  ffi = require("ffi")

  -- Rustで作成した関数を定義
  ffi.cdef [[
    int fn_name1(int a, int b);
    uint32_t fn_name2(const uint32_t* args, size_t len);
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
