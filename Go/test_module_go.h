/* Code generated by cmd/cgo; DO NOT EDIT. */

/* package command-line-arguments */


#line 1 "cgo-builtin-export-prolog"

#include <stddef.h>

#ifndef GO_CGO_EXPORT_PROLOGUE_H
#define GO_CGO_EXPORT_PROLOGUE_H

#ifndef GO_CGO_GOSTRING_TYPEDEF
typedef struct { const char *p; ptrdiff_t n; } _GoString_;
#endif

#endif

/* Start of preamble from import "C" comments.  */


#line 3 "test_module_go.go"

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

#line 1 "cgo-generated-wrapper"


/* End of preamble from import "C" comments.  */


/* Start of boilerplate cgo prologue.  */
#line 1 "cgo-gcc-export-header-prolog"

#ifndef GO_CGO_PROLOGUE_H
#define GO_CGO_PROLOGUE_H

typedef signed char GoInt8;
typedef unsigned char GoUint8;
typedef short GoInt16;
typedef unsigned short GoUint16;
typedef int GoInt32;
typedef unsigned int GoUint32;
typedef long long GoInt64;
typedef unsigned long long GoUint64;
typedef GoInt64 GoInt;
typedef GoUint64 GoUint;
typedef size_t GoUintptr;
typedef float GoFloat32;
typedef double GoFloat64;
#ifdef _MSC_VER
#include <complex.h>
typedef _Fcomplex GoComplex64;
typedef _Dcomplex GoComplex128;
#else
typedef float _Complex GoComplex64;
typedef double _Complex GoComplex128;
#endif

/*
  static assertion to make sure the file is being used on architecture
  at least with matching size of GoInt.
*/
typedef char _check_for_64_bit_pointer_matching_GoInt[sizeof(void*)==64/8 ? 1:-1];

#ifndef GO_CGO_GOSTRING_TYPEDEF
typedef _GoString_ GoString;
#endif
typedef void *GoMap;
typedef void *GoChan;
typedef struct { void *t; void *v; } GoInterface;
typedef struct { void *data; GoInt len; GoInt cap; } GoSlice;

#endif

/* End of boilerplate cgo prologue.  */

#ifdef __cplusplus
extern "C" {
#endif

extern __declspec(dllexport) RGBAs* GetRGBAs(char* input);
extern __declspec(dllexport) void FreeRGBAs(RGBAs* rgbasPtr);

#ifdef __cplusplus
}
#endif
