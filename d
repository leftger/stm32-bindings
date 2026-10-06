#!/usr/bin/env bash

set -e
cd $(dirname $0)

CMD=$1
WBA_REV=v1.10.0
WB_REV=v1.24.0
XCUBE_IP_REV=v1.0.0
N6_REV=v1.0.0
shift

fetch_nema_gfx() {
    local tmp
    tmp=$(mktemp -d)
    git clone --depth 1 https://github.com/STMicroelectronics/x-cube-image-processing.git "$tmp"
    cd "$tmp"
    git fetch origin "$XCUBE_IP_REV" 2>/dev/null || true
    git checkout FETCH_HEAD 2>/dev/null || git checkout "$XCUBE_IP_REV" 2>/dev/null || git checkout main
    cd - >/dev/null

    rm -rf ./stm32-bindings-gen/nema_gfx/include
    mkdir -p ./stm32-bindings-gen/nema_gfx/include ./stm32-bindings-gen/nema_gfx/lib
    cp -R "$tmp/Middleware/NemaGFX/include/." ./stm32-bindings-gen/nema_gfx/include/
    cp "$tmp/Middleware/NemaGFX/LICENSE.md" ./stm32-bindings-gen/nema_gfx/LICENSE.md
    cat > ./stm32-bindings-gen/nema_gfx/VERSION <<EOF
x-cube-image-processing ${XCUBE_IP_REV}
NemaGFX middleware v1.4.17
EOF

    local src dst
    for pair in \
        "cortex_m33_revC:cortex_m33_revc" \
        "cortex_m33_NemaPVG:cortex_m33_nemapvg" \
        "cortex_m7:cortex_m7" \
        "cortex_m55:cortex_m55"
    do
        src="${pair%%:*}"
        dst="${pair##*:}"
        cp "$tmp/Middleware/NemaGFX/lib/core/${src}/gcc/libnemagfx-float-abi-hard.a" \
            "./stm32-bindings-gen/nema_gfx/lib/libnemagfx_${dst}_float_abi_hard.a"
    done

    rm -rf "$tmp"
    echo "NemaGFX headers and libraries updated under stm32-bindings-gen/nema_gfx/"
}

# STM32CubeN6 provides the VideoEncoder middleware as source only (no prebuilt
# archives), so unlike NemaGFX we compile `libvenc` ourselves. The sources live
# in Middlewares/Third_Party/VideoEncoder; the public headers + the portability
# shim (stm32-bindings-gen/venc_port.c) are all that is needed.
#
# Use STM32CUBEN6_DIR to point at an existing checkout, otherwise ./d fetch-n6
# clones one into sources/STM32CubeN6.
build_venc() {
    local n6="${STM32CUBEN6_DIR:-sources/STM32CubeN6}"
    local src="$n6/Middlewares/Third_Party/VideoEncoder"
    local out=./stm32-bindings-gen/venc

    if [ ! -d "$src" ]; then
        echo "STM32CubeN6 VideoEncoder not found at $src" >&2
        echo "Run './d fetch-n6' first, or set STM32CUBEN6_DIR." >&2
        exit 1
    fi

    rm -rf "$out"
    mkdir -p "$out/include" "$out/lib"
    cp "$src"/inc/basetype.h "$src"/inc/enccommon.h "$src"/inc/ewl.h \
       "$src"/inc/h264encapi.h "$src"/inc/h264encapi_ext.h "$src"/inc/jpegencapi.h \
       "$out/include/"
    cp "$src/LICENSE.md" "$out/LICENSE.md"

    local tmp
    tmp=$(mktemp -d)
    local cflags=(-mcpu=cortex-m55 -mthumb -mfpu=fpv5-d16 -mfloat-abi=hard -O2
                  -ffunction-sections -fdata-sections
                  -I"$src"/inc -I"$src"/source/common -I"$src"/source/h264 -I"$src"/source/jpeg)

    local f b
    for f in "$src"/source/common/*.c "$src"/source/h264/*.c "$src"/source/jpeg/*.c; do
        b=$(basename "${f%.c}")
        # H264TestId.c is ST's internal test-vector code. venc_port.c provides the
        # two hooks it exports, avoiding its stdio/malloc dependencies.
        [ "$b" = "H264TestId" ] && continue
        arm-none-eabi-gcc -c "${cflags[@]}" "$f" -o "$tmp/$b.o"
    done
    arm-none-eabi-gcc -c "${cflags[@]}" ./stm32-bindings-gen/venc_port.c -o "$tmp/venc_port.o"
    arm-none-eabi-ar rcs "$out/lib/libvenc_cortex_m55_float_abi_hard.a" "$tmp"/*.o
    rm -rf "$tmp"

    cat > "$out/VERSION" <<EOF
STM32CubeN6 ${N6_REV}
Middlewares/Third_Party/VideoEncoder (Hantro H.264 + JPEG encoder)
built with arm-none-eabi-gcc, -O2 -mcpu=cortex-m55 -mfpu=fpv5-d16 -mfloat-abi=hard
EOF
    echo "VENC headers and library updated under stm32-bindings-gen/venc/"
}

fetch_n6() {
    local dest="${STM32CUBEN6_DIR:-sources/STM32CubeN6}"
    if [ ! -d "$dest/Middlewares/Third_Party/VideoEncoder" ]; then
        # STM32CubeN6 is large; only the VideoEncoder middleware is needed.
        git clone --depth 1 --filter=blob:none --sparse \
            https://github.com/STMicroelectronics/STM32CubeN6.git "$dest"
        git -C "$dest" sparse-checkout set Middlewares/Third_Party/VideoEncoder
    fi
}

case "$CMD" in
    gen)
        cargo run --release --bin stm32-bindings-gen
    ;;
    download-all)
        rm -rf ./sources
        git clone https://github.com/STMicroelectronics/STM32CubeWBA.git ./sources/STM32CubeWBA/ --depth 1
        git clone https://github.com/STMicroelectronics/STM32CubeWB.git ./sources/STM32CubeWB/ --depth 1
        cd ./sources/STM32CubeWBA/
        git fetch origin $WBA_REV
        git checkout FETCH_HEAD
        git submodule update --init --recursive
        cd ../..
        cd ./sources/STM32CubeWB/
        git fetch origin $WB_REV
        git checkout FETCH_HEAD
        git submodule update --init --recursive
        cd ../..
        fetch_nema_gfx
        fetch_n6
        build_venc
    ;;
    build-thread)
        mkdir -p build
        rm -rf build/thread 
        mkdir -p build/thread
        
        cp -r stm32-bindings-gen/thread build
        cd build/thread

        cmake -B build -G Ninja -DCMAKE_C_COMPILER=arm-none-eabi-gcc -DCMAKE_BUILD_TYPE=Release "-DCMAKE_TOOLCHAIN_FILE=arm-gcc-toolchain.cmake"
        cmake --build build

        cd ../..
    ;;
    fetch-nema-gfx)
        fetch_nema_gfx
    ;;
    fetch-n6)
        fetch_n6
    ;;
    build-venc)
        build_venc
    ;;
    *)
        echo "unknown command"
    ;;
esac
