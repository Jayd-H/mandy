@echo off
echo ========================================
echo Mandy MD to PDF Converter - Full Build
echo ========================================
echo.

echo Cleaning previous builds...
cargo clean
if %ERRORLEVEL% NEQ 0 (
    echo Failed to clean!
    pause
    exit /b 1
)
echo.

echo Building installer (with manifest and icon)...
cargo build --release --bin mandy-installer
if %ERRORLEVEL% NEQ 0 (
    echo.
    echo Installer build failed!
    pause
    exit /b 1
)
echo.

echo Building converter (with icon)...
cargo build --release --bin mandy-converter
if %ERRORLEVEL% NEQ 0 (
    echo.
    echo Converter build failed!
    pause
    exit /b 1
)

echo.
echo ========================================
echo Build successful!
echo ========================================
echo.
echo Executables created:
echo - target\release\mandy-installer.exe
echo - target\release\mandy-converter.exe
echo.
echo IMPORTANT: Check the build output above for these warnings:
echo   - "BUILD.RS IS RUNNING"
echo   - "Setting manifest for installer"
echo   - "Resources compiled successfully"
echo.
echo If you don't see those warnings, build.rs is not in the project root!
echo.
pause