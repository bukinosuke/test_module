// ※dll→dllになるためかクラッシュするので利用不可

if lib == nil then
  ffi = require("ffi")

  -- Rustで作成した関数を定義
  ffi.cdef [[
    char* fn_name7(void);
  ]]

  -- DLLの読み込み
  lib = ffi.load("E:/Documents/Training/Test/test_module/target/release/test_module.dll")
end

-- Lua → Rust → Go の順で処理を呼び、値を受け取る
local fn_name7 = lib.fn_name7()
-- ポインタから文字列を取得
local string = ffi.string(fn_name7)
print(string)
-- fn_name7で作成した文字列のメモリをモジュール内から解放
lib.free_string(fn_name7)
