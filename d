#!/usr/bin/env bash

set -e
cd $(dirname $0)

CMD=$1
WBA_REV=v1.9.0
WB_REV=v1.24.0
shift

case "$CMD" in
    gen)
        cargo run --release stm32-bindings-gen
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
    ;;
    *)
        echo "unknown command"
    ;;
esac
