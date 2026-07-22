/**
 * Minimal GPU2D HAL declarations used by NemaGFX platform HAL.
 *
 * Replace these with STM32Cube HAL when running on hardware: initialize
 * `hgpu2d`, enable clocks, and link the real HAL_GPU2D_* implementation
 * instead of the stub in `gpu2d_hal_stub.c` (disable the `stub` feature).
 */
#ifndef HAL_GPU2D_SHIM_H__
#define HAL_GPU2D_SHIM_H__

#include <stdint.h>

#ifndef UNUSED
#define UNUSED(x) ((void)(x))
#endif

typedef enum {
    HAL_OK = 0x00U,
    HAL_ERROR = 0x01U,
} HAL_StatusTypeDef;

typedef struct {
    void *Instance;
} GPU2D_HandleTypeDef;

/* GPU2D register offset used by NemaGFX HAL for breakpoint polling. */
#define GPU2D_BREAKPOINT 0x00000020U

extern GPU2D_HandleTypeDef hgpu2d;

HAL_StatusTypeDef HAL_GPU2D_Init(GPU2D_HandleTypeDef *hgpu2d);
uint32_t HAL_GPU2D_ReadRegister(GPU2D_HandleTypeDef *hgpu2d, uint32_t reg);
void HAL_GPU2D_WriteRegister(GPU2D_HandleTypeDef *hgpu2d, uint32_t reg, uint32_t value);

#if defined(USE_HAL_GPU2D_REGISTER_CALLBACKS) && (USE_HAL_GPU2D_REGISTER_CALLBACKS == 1)
typedef void (*GPU2D_CommandListCpltCallbackTypeDef)(GPU2D_HandleTypeDef *hgpu2d,
                                                       uint32_t CmdListID);
HAL_StatusTypeDef HAL_GPU2D_RegisterCommandListCpltCallback(
    GPU2D_HandleTypeDef *hgpu2d, GPU2D_CommandListCpltCallbackTypeDef cb);
#endif

#endif /* HAL_GPU2D_SHIM_H__ */
