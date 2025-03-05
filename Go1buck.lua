if ffi == nil then
  ffi = require("ffi")
end

if go_lib == nil then
  -- Goで作成した関数を定義
  ffi.cdef [[
    char* goFunction(const char* str);
    void goFree(char* ptr);
  ]]

  -- DLLの読み込み
  go_lib = ffi.load("E:/Documents/Training/Test/test_module/Go/test_module_go.dll")
end

-- モジュール関数に文字列を渡し、文字列を受け取る
local str = "ウンチーコングって・・・"
local result = go_lib.goFunction(str)
-- ポインタから文字列を取得
local string = ffi.string(result)
print(string)
-- goFunctionで作成した文字列のメモリをモジュール内から解放
go_lib.goFree(result)
