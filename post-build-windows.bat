@echo off
REM Post-build script for Windows to bundle Npcap DLLs with the app
REM This ensures the app works on machines without Npcap installed

echo 🪟 Windows Post-Build: Bundling Npcap with app...

REM Configuration
set APP_NAME=InnoMonitor
set BUNDLE_PATH=src-tauri\target\release\bundle\msi
set MSI_DIR=src-tauri\target\release
set BINARY_PATH=%MSI_DIR%\%APP_NAME%.exe

REM Check if the binary exists
if not exist "%BINARY_PATH%" (
    echo ❌ Binary not found at: %BINARY_PATH%
    echo    Please run 'npm run tauri build' first
    exit /b 1
)

echo ✅ Found binary at: %BINARY_PATH%

REM Create libs directory for bundled DLLs
set LIBS_DIR=%MSI_DIR%\libs
if not exist "%LIBS_DIR%" (
    echo 📁 Creating libs directory...
    mkdir "%LIBS_DIR%"
)

REM Find Npcap DLLs on the system
set FOUND_WPCAP=0
set FOUND_PACKET=0

REM Check common Npcap installation paths
set NPCAP_PATHS[0]=C:\Windows\System32\Npcap
set NPCAP_PATHS[1]=C:\Windows\SysWOW64\Npcap
set NPCAP_PATHS[2]=C:\Program Files\Npcap
set NPCAP_PATHS[3]=C:\Program Files (x86)\Npcap

for %%i in (0 1 2 3) do (
    call set "NPCAP_PATH=%%NPCAP_PATHS[%%i]%%"
    call :check_npcap_path "!NPCAP_PATH!"
)

if %FOUND_WPCAP%==0 (
    echo ❌ wpcap.dll not found in any standard Npcap location!
    echo    Please install Npcap from: https://npcap.com/
    echo    Standard locations checked:
    echo      - C:\Windows\System32\Npcap
    echo      - C:\Windows\SysWOW64\Npcap  
    echo      - C:\Program Files\Npcap
    echo      - C:\Program Files (x86)\Npcap
    exit /b 1
)

if %FOUND_PACKET%==0 (
    echo ⚠️  Packet.dll not found - app may have runtime issues
)

echo 🎉 Windows Post-Build Complete!
echo.
echo 📦 Your app bundle now includes:
echo    • Bundled Npcap DLLs in: %LIBS_DIR%
echo    • Application binary: %BINARY_PATH%
echo.
echo ✅ The app should now work on Windows machines without Npcap installed
echo.
echo 🧪 To test on another machine:
echo    1. Copy the entire release directory: %MSI_DIR%
echo    2. Run: %APP_NAME%.exe
echo    3. Check for any DLL loading errors
echo.

goto :eof

:check_npcap_path
set "NPCAP_PATH=%~1"
if exist "%NPCAP_PATH%\wpcap.dll" (
    echo ✅ Found wpcap.dll at: %NPCAP_PATH%
    copy "%NPCAP_PATH%\wpcap.dll" "%LIBS_DIR%\" >nul 2>&1
    if %errorlevel%==0 (
        echo 📦 Copied wpcap.dll to app bundle
        set FOUND_WPCAP=1
    ) else (
        echo ⚠️  Failed to copy wpcap.dll
    )
)

if exist "%NPCAP_PATH%\Packet.dll" (
    echo ✅ Found Packet.dll at: %NPCAP_PATH%
    copy "%NPCAP_PATH%\Packet.dll" "%LIBS_DIR%\" >nul 2>&1
    if %errorlevel%==0 (
        echo 📦 Copied Packet.dll to app bundle
        set FOUND_PACKET=1
    ) else (
        echo ⚠️  Failed to copy Packet.dll
    )
)
goto :eof
