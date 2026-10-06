/* STM32N6 Video Encoder (VENC) — Hantro H.264 + JPEG encoder API.
 *
 * Umbrella header for bindgen. Pulls in the two public encoder APIs plus the
 * Encoder Wrapper Layer (EWL) types they depend on:
 *   - h264encapi.h / h264encapi_ext.h : H.264 encoder API
 *   - jpegencapi.h                     : JPEG encoder API
 *   - enccommon.h -> ewl.h             : EWL types (EWLLinearMem_t, ...)
 */
#include "h264encapi.h"
#include "h264encapi_ext.h"
#include "jpegencapi.h"
