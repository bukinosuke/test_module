package main

/*
// Cとの連携用ヘッダー宣言
#include <stdlib.h>
#include <stdint.h>
#include <string.h>

typedef struct {
    uint8_t* r;    // 赤成分配列
    int r_len;     // 赤成分の要素数
    uint8_t* g;    // 緑成分配列
    int g_len;     // 緑成分の要素数
    uint8_t* b;    // 青成分配列
    int b_len;     // 青成分の要素数
    uint8_t* a;    // アルファ成分配列
    int a_len;     // アルファ成分の要素数
    int len;       // 各配列の共通の長さ
    char* name;    // 文字列（例：画像名）
    int x;         // X座標（例）
    int y;         // Y座標（例）
} SimpleRGBA_C;

typedef struct {
    SimpleRGBA_C* data; // SimpleRGBA_C構造体の配列
    int count;          // 要素数
} RGBAs;
*/
import "C"
import (
	"os"
	"unsafe"

	"github.com/oov/psd"
)

// Go側のデータ構造
type SimpleRGBA struct {
	r    []uint8
	g    []uint8
	b    []uint8
	a    []uint8
	len  int
	name string
	x    int
	y    int
}

// copyToC は、Goの []uint8 の内容を Cのヒープにコピーします。
func copyToC(data []uint8) unsafe.Pointer {
	if len(data) == 0 {
		return nil
	}
	size := C.size_t(len(data))
	ptr := C.malloc(size)
	C.memcpy(ptr, unsafe.Pointer(&data[0]), size)
	return ptr
}

// setByteField は、Goのスライスの内容を Cのフィールドへセットします。
func setByteField(field **C.uint8_t, data []uint8) {
	*field = (*C.uint8_t)(copyToC(data))
}

//export GetRGBAs
func GetRGBAs(input *C.char) *C.RGBAs {
	path := C.GoString(input)

	// PSDファイルを解析
	file, err := os.Open(path)
	if err != nil {
		panic(err)
	}
	defer file.Close()

	// ↑で取得したPSDファイルを解析
	img, _, err := psd.Decode(file, &psd.DecodeOptions{SkipLayerImage: false, SkipMergedImage: false})
	if err != nil {
		panic(err)
	}

	// RGBAの取得
	var r []uint8
	for _, value := range img.Channel[0].Data {
		r = append(r, uint8(value))
	}
	var g []uint8
	for _, value := range img.Channel[1].Data {
		g = append(g, uint8(value))
	}
	var b []uint8
	for _, value := range img.Channel[2].Data {
		b = append(b, uint8(value))
	}
	var a []uint8
	for _, value := range img.Channel[-1].Data {
		a = append(a, uint8(value))
	}

	rgba := SimpleRGBA{
		r:    r,
		g:    g,
		b:    b,
		a:    a,
		len:  len(r),
		name: "image",
		x:    img.Config.Rect.Size().X,
		y:    img.Config.Rect.Size().Y,
	}

	// 複数の要素を返す場合、必要に応じて追加してください
	rgbasGo := []SimpleRGBA{rgba}
	count := len(rgbasGo)

	// --- C側用のメモリ確保 ---
	// SimpleRGBA_C構造体の配列を一括で確保
	size := C.size_t(count) * C.size_t(unsafe.Sizeof(C.SimpleRGBA_C{}))
	dataPtr := C.malloc(size)
	// unsafe.Slice を使い、dataPtr から長さ count のスライスを生成
	cArray := unsafe.Slice((*C.SimpleRGBA_C)(dataPtr), count)

	// 各要素のデータをCヒープへコピーしてセット
	for i, item := range rgbasGo {
		setByteField(&cArray[i].r, item.r)
		cArray[i].r_len = C.int(len(item.r))

		setByteField(&cArray[i].g, item.g)
		cArray[i].g_len = C.int(len(item.g))

		setByteField(&cArray[i].b, item.b)
		cArray[i].b_len = C.int(len(item.b))

		setByteField(&cArray[i].a, item.a)
		cArray[i].a_len = C.int(len(item.a))

		cArray[i].len = C.int(item.len)
		cArray[i].name = C.CString(item.name)
		cArray[i].x = C.int(item.x)
		cArray[i].y = C.int(item.y)
	}

	// RGBAs構造体自体も C.malloc で確保
	rgbasSize := C.size_t(unsafe.Sizeof(C.RGBAs{}))
	rgbasPtr := (*C.RGBAs)(C.malloc(rgbasSize))
	rgbasPtr.data = (*C.SimpleRGBA_C)(dataPtr)
	rgbasPtr.count = C.int(count)

	return rgbasPtr
}

// freeField は、ポインタが nil でなければ C.free で解放します。
func freeField(ptr unsafe.Pointer) {
	if ptr != nil {
		C.free(ptr)
	}
}

//export FreeRGBAs
func FreeRGBAs(rgbasPtr *C.RGBAs) {
	if rgbasPtr == nil {
		return
	}
	count := int(rgbasPtr.count)
	// unsafe.Slice を利用して、確保した構造体配列を安全に扱う
	cArray := unsafe.Slice(rgbasPtr.data, count)
	for i := 0; i < count; i++ {
		freeField(unsafe.Pointer(cArray[i].r))
		freeField(unsafe.Pointer(cArray[i].g))
		freeField(unsafe.Pointer(cArray[i].b))
		freeField(unsafe.Pointer(cArray[i].a))
		freeField(unsafe.Pointer(cArray[i].name))
	}
	// 配列とRGBAs構造体自体のメモリを解放
	C.free(unsafe.Pointer(rgbasPtr.data))
	C.free(unsafe.Pointer(rgbasPtr))
}

func main() {}
