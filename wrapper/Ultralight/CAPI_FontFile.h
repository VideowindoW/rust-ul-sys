#ifndef ULTRALIGHT_CAPI_FONTFILE_H
#define ULTRALIGHT_CAPI_FONTFILE_H

#include "CAPI_Defines.h"
#include "CAPI_String.h"
#include "CAPI_Buffer.h"

#ifdef __cplusplus
extern "C" {
#endif

///
/// Create a font file from an on-disk file path.
///
/// @note  The file path should already exist.
///
ULExport ULFontFile ulFontFileCreateFromFilePath(ULString file_path);

///
/// Create a font file from an in-memory buffer.
///
ULExport ULFontFile ulFontFileCreateFromBuffer(ULBuffer buffer);

///
/// Destroy font file
///
ULExport void ulDestroyFontFile(ULFontFile font_file);

#ifdef __cplusplus
} // extern "C"
#endif

#endif // ULTRALIGHT_CAPI_FONTFILE_H