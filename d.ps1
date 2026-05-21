<# #>
param (
    [Parameter(Mandatory = $true)]
    [string]$CMD,

    [string]$peri
)

$WBA_REV = ((Select-String -Path ".\d" -Pattern "^WBA_REV=") -split "=")[1]
$WB_REV = ((Select-String -Path ".\d" -Pattern "^WB_REV=") -split "=")[1]

Switch ($CMD) {
    "gen" {
        cargo run --release stm32-bindings-gen --target thumbv8m.main-none-eabihf
    }
    "download-all" {
        while (Test-Path "sources") {
            rm -r -Force sources -ErrorAction SilentlyContinue
        }

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
    }
    default {
        echo "unknown command"
    }
}