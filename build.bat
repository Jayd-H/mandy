@echo off
echo Building Mandy MD to PDF Converter...
echo.

echo Building installer...
cargo build --release --bin mandy-installer
if %ERRORLEVEL% NEQ 0 (
    echo.
    echo Installer build failed!
    pause
    exit /b 1
)

echo Building converter...
cargo build --release --bin mandy-converter
if %ERRORLEVEL% NEQ 0 (
    echo.
    echo Converter build failed!
    pause
    exit /b 1
)

echo.
echo Build successful!
echo.
echo Executables created:
echo - target\release\mandy-installer.exe
echo - target\release\mandy-converter.exe
echo.
echo Copy both files to the same folder for distribution.
echo.
pause