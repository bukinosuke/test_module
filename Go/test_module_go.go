package main

// #include <stdlib.h>
import "C"
import (
	"unsafe"
)

//export goFunction
func goFunction(input *C.char) *C.char {
	// C互換の文字列をGo文字列に変換
	rust_str := C.GoString(input)

	// 文字列を加工
	str := rust_str + "知らないよｗｗｗ"

	// 加工後の文字列をC互換の文字列に変換
	return C.CString(str) // メモリ確保
}

//export goFree
func goFree(ptr *C.char) {
	// メモリ解放
	C.free(unsafe.Pointer(ptr))
}

func main() {}
