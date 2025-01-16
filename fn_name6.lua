if lib == nil then
  ffi = require("ffi")

  -- 構造体の定義、関数の定義、ポインタ解放関数の定義
  ffi.cdef [[
    typedef struct {
      const char** ptr;
      size_t len;
    } StringArrayResult;

    StringArrayResult fn_name6(const char* str);

    void free_string_array(StringArrayResult result);
  ]]

  lib = ffi.load("E:/Documents/Training/Test/test_module/target/release/test_module.dll")
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
