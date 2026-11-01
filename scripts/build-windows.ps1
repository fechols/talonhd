# Build the C64 program from Windows through WSL. Bake assets first with
#   cargo run --release -- bake
param([string]$Distribution = 'Ubuntu-24.04', [string]$Features = '')
$ErrorActionPreference = 'Stop'
$projectRoot = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot '..')).Path
$linuxRoot = (& wsl -d $Distribution -- wslpath -a -u $projectRoot.Replace('\', '/')).Trim()
if ($LASTEXITCODE -ne 0) { throw 'Cannot access the WSL build environment.' }
if ($Features) {
    & wsl -d $Distribution -- bash "$linuxRoot/scripts/build-target.sh" --features $Features
} else {
    & wsl -d $Distribution -- bash "$linuxRoot/scripts/build-target.sh"
}
if ($LASTEXITCODE -ne 0) { throw 'TALON HD C64 build failed.' }
