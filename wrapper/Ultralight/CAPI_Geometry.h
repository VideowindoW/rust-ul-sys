#ifndef ULTRALIGHT_CAPI_GEOMETRY_H
#define ULTRALIGHT_CAPI_GEOMETRY_H

#include "CAPI_Defines.h"

#ifdef __cplusplus
extern "C" {
#endif

/******************************************************************************
 * Rect
 *****************************************************************************/

///
/// Whether or not a ULRect is empty (all members equal to 0)
///
ULExport bool ulRectIsEmpty(ULRect rect);

///
/// Create an empty ULRect (all members equal to 0)
///
ULExport ULRect ulRectMakeEmpty();

/******************************************************************************
 * IntRect
 *****************************************************************************/

///
/// Whether or not a ULIntRect is empty (all members equal to 0)
///
ULExport bool ulIntRectIsEmpty(ULIntRect rect);

///
/// Create an empty ULIntRect (all members equal to 0)
///
ULExport ULIntRect ulIntRectMakeEmpty();

#ifdef __cplusplus
} // extern "C"
#endif

#endif // ULTRALIGHT_CAPI_GEOMETRY_H