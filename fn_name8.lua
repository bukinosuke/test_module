// 共有変数のポインタを渡すコード、クローン不要

-- Rustで作成した関数を定義
if lib == nil then
  ffi.cdef [[
    typedef struct {
      const uint32_t* r;
      const uint32_t* g;
      const uint32_t* b;
      const uint32_t* a;
      const bool* w;
      size_t len;
    }RGBAW;

    RGBAW fn_name8(void);
  ]]

  -- DLLの読み込み
  lib = ffi.load("E:\\Documents\\Training\\Test\\test_module\\target\\release\\test_module.dll")
end

-- Rustのグローバル変数へとアクセス
local rgba = lib.fn_name8()

-- 値の確認
print("r")
for i = 0, tonumber(rgba.len) - 1 do
  print(rgba.r[i])
end
print("g")
for i = 0, tonumber(rgba.len) - 1 do
  print(rgba.g[i])
end
print("b")
for i = 0, tonumber(rgba.len) - 1 do
  print(rgba.b[i])
end
print("a")
for i = 0, tonumber(rgba.len) - 1 do
  print(rgba.a[i])
end
print("w")
for i = 0, tonumber(rgba.len) - 1 do
  print(rgba.w[i])
end
