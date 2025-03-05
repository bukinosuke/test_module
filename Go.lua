local ffi = require("ffi")

-- Go側で定義した構造体とエクスポート関数の宣言
ffi.cdef[[
typedef unsigned char uint8_t;

typedef struct {
    uint8_t* r;
    int r_len;
    uint8_t* g;
    int g_len;
    uint8_t* b;
    int b_len;
    uint8_t* a;
    int a_len;
    int len;
    char* name;
    int x;
    int y;
} SimpleRGBA_C;

typedef struct {
    SimpleRGBA_C* data;
    int count;
} RGBAs;

RGBAs* GetRGBAs();
void FreeRGBAs(RGBAs* rgbas);
]]

-- DLLの読み込み（環境に合わせてパス・名前を変更）
local lib = ffi.load("mydll")  -- 例: Windowsなら "mydll.dll"

-- GetRGBAsでデータ取得
local rgbas = lib.GetRGBAs()
local count = rgbas.count
print("取得した要素数:", count)

-- ffi.castでC側の配列として扱う
local rgbaArray = ffi.cast("SimpleRGBA_C*", rgbas.data)

-- printArray は各色成分の配列内容を文字列化して表示します
local function printArray(arr, len, label)
  local parts = {}
  for j = 0, len - 1 do
    parts[#parts + 1] = arr[j]
  end
  print(label .. ": " .. table.concat(parts, ", "))
end

for i = 0, count - 1 do
  local rgba = rgbaArray[i]
  local name = ffi.string(rgba.name)
  print(string.format("要素[%d] - Name: %s, x: %d, y: %d, len: %d", i, name, rgba.x, rgba.y, rgba.len))

  printArray(rgba.r, rgba.r_len, "R")
  printArray(rgba.g, rgba.g_len, "G")
  printArray(rgba.b, rgba.b_len, "B")
  printArray(rgba.a, rgba.a_len, "A")
  print("---------------------------------")
end

-- 使用後は必ずメモリ解放
lib.FreeRGBAs(rgbas)
