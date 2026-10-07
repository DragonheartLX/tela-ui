@echo off
setlocal enabledelayedexpansion

set "LIGHT=../docs/screenshot-light.png"
set "DARK=../docs/screenshot-dark.png"
set "OUT=../docs/screenshot.png"

set "W=1920"
set "H=1080"
set "DIR=/"

slint-viewer -L "tela-ui=%~dp0..\tela-ui" --style fluent-light --size 1920x1080 --screenshot "%~dp0..\docs\screenshot-light.png" "%~dp0..\ui\MainWindow.slint"
slint-viewer -L "tela-ui=%~dp0..\tela-ui" --style fluent-dark --size 1920x1080 --screenshot "%~dp0..\docs\screenshot-dark.png" "%~dp0..\ui\MainWindow.slint"

if "%DIR%"=="/" (
    set "EXPR=if(gt(X/W+Y/H,1),A,B)"
) else (
    set "EXPR=if(gt(X/W-Y/H,0),A,B)"
)

set "SZ=!W!:!H!"

echo Converting...
echo   LIGHT: %LIGHT%
echo   DARK : %DARK%
echo   OUT  : %OUT%
echo   SIZE : !SZ!
echo.

ffmpeg -y -i "%LIGHT%" -i "%DARK%" -filter_complex "[0:v]scale=!SZ!:force_original_aspect_ratio=decrease,pad=!SZ!:(ow-iw)/2:(oh-ih)/2:color=black[s0];[1:v]scale=!SZ!:force_original_aspect_ratio=decrease,pad=!SZ!:(ow-iw)/2:(oh-ih)/2:color=black[s1];[s0][s1]blend=all_expr='!EXPR!'[out]" -map "[out]" -frames:v 1 "%OUT%"

if errorlevel 1 (
    echo.
    echo error
) else (
    echo.
    echo done: %OUT%
)

pause
endlocal