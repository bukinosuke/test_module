if lib == nil then
    ffi = require("ffi")

    -- Rustで作成した関数を定義
    ffi.cdef [[
        typedef struct {
            const uint32_t* ptr;
            size_t len;
        } VecResult;

        typedef struct {
            const char** ptr;
            size_t len;
        } StringArrayResult;

        int fn_name1(int a, int b);
        uint32_t fn_name2(const uint32_t* args, size_t len);

        VecResult fn_name3(const uint32_t* args, size_t len);
        void free_name3(const uint32_t* ptr, size_t len);
        
        char* fn_name4(const char* str);
        void free_string(char* ptr);

        StringArrayResult fn_name5(
        const char** input,
        size_t len
        );
        void free_string_array(StringArrayResult result);

    ]]

    -- DLLの読み込み
    lib = ffi.load("E:/Documents/Training/Test/test_module/target/release/test_module.dll")

    print(lib)
end

-- モジュール関数の呼び出し
local fn_name1 = lib.fn_name1(1, 2)
print(fn_name1)

-- モジュール関数に配列を渡す
local values2 = ffi.new("uint32_t[5]", { 10, 20, 30, 40, 50 })
local fn_name2 = lib.fn_name2(values2, 5)
print(fn_name2)

-- モジュール関数に配列を渡し、配列を受け取る
local values3 = ffi.new("uint32_t[3]", { 11, 22, 33 })
local fn_name3 = lib.fn_name3(values3, 3)
print(fn_name3)
-- 受け取った値のポインタと長さを利用してLuaのテーブルに変換
local output3 = {}
for i = 0, tonumber(fn_name3.len) - 1 do
    table.insert(output3, fn_name3.ptr[i])
end
-- ポインタのメモリ領域を解放
print(output3)
lib.free_name3(fn_name3.ptr, fn_name3.len)
-- テーブルを文字列にして可視化
local output3_string = ""
for i, value in ipairs(output3) do
    output3_string = output3_string .. value
end
print(output3_string)

-- モジュール関数に文字列を渡し、文字列を受け取る
local str = "知ってる？"
local fn_name4 = lib.fn_name4(str)
-- ポインタから文字列を取得
local output4 = ffi.string(fn_name4)
print(output4)
-- fn_name4で作成した文字列のメモリをモジュール内から解放
lib.free_string(fn_name4)

-- モジュール関数に文字列配列を渡し、文字列配列を受け取る
local input = ffi.new("const char*[3]", { "hello", "world", "lua" })
local fn_name5 = lib.fn_name5(input, 3)
-- 結果をLuaのテーブルに変換
local output5 = {}
for i = 0, tonumber(fn_name5.len) - 1 do
    table.insert(output5, ffi.string(fn_name5.ptr[i]))
end
-- 出力を表示
print(table.unpack(output5))
-- fn_name5で作成した文字列配列のメモリをモジュール内から解放
lib.free_string_array(fn_name5)