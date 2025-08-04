@echo off
setlocal enabledelayedexpansion
REM Pre-build script for Windows to copy Npcap DLLs before Tauri build
REM This ensures Tauri includes the DLLs as resources

echo 🪟 Windows Pre-Build: Preparing Npcap DLLs for bundling...

REM Create libs directory in src-tauri for Tauri to pick up as resources
set "LIBS_DIR=src-tauri\libs"
if not exist "%LIBS_DIR%" (
    echo 📁 Creating libs directory for Tauri resources...
    mkdir "%LIBS_DIR%"
)

REM Check if we have pre-bundled runtime DLLs
set "RUNTIME_DIR=src-tauri\npcap-runtime"
if exist "%RUNTIME_DIR%\wpcap.dll" (
    echo ✅ Found pre-bundled Npcap runtime DLLs
    copy "%RUNTIME_DIR%\wpcap.dll" "%LIBS_DIR%\" >nul 2>&1
    if !errorlevel!==0 (
        echo 📦 Copied wpcap.dll from bundled runtime
        set FOUND_WPCAP=1
    )
)

if exist "%RUNTIME_DIR%\Packet.dll" (
    copy "%RUNTIME_DIR%\Packet.dll" "%LIBS_DIR%\" >nul 2>&1
    if !errorlevel!==0 (
        echo 📦 Copied Packet.dll from bundled runtime
        set FOUND_PACKET=1
    )
)

REM If we found bundled DLLs, we're done
if defined FOUND_WPCAP (
    if defined FOUND_PACKET (
        echo ✅ Using pre-bundled Npcap runtime DLLs
        goto :success
    )
)

echo ⚠️  No pre-bundled runtime DLLs found
echo    Trying to copy from system installation...

REM Find and copy Npcap DLLs from system installation
set FOUND_WPCAP=0
set FOUND_PACKET=0
set FOUND_RUNTIME_DLLS=0

REM Check common Npcap installation paths
call :check_and_copy_npcap "C:\Windows\System32\Npcap"
call :check_and_copy_npcap "C:\Windows\SysWOW64\Npcap"
call :check_and_copy_npcap "C:\Program Files\Npcap"
call :check_and_copy_npcap "C:\Program Files (x86)\Npcap"

if %FOUND_WPCAP%==0 (
    echo ❌ wpcap.dll not found in any standard Npcap location!
    echo    Please install Npcap from: https://npcap.com/
    echo    Standard locations checked:
    echo      - C:\Windows\System32\Npcap
    echo      - C:\Windows\SysWOW64\Npcap
    echo      - C:\Program Files\Npcap
    echo      - C:\Program Files (x86)\Npcap
    echo.
    echo ⚠️  Build will continue, but the installer may not work on machines without Npcap
    goto :eof
)

:success
echo.
echo 🎉 Pre-Build Complete!
echo.
echo 📦 DLLs prepared for bundling:
dir /b "%LIBS_DIR%\*.dll" 2>nul
echo.
echo ✅ Tauri will now include these DLLs in the installer
echo.

goto :eof

:copy_runtime_dlls_from_system
REM Try to get runtime DLLs from system installation
call :check_and_copy_npcap "C:\Windows\System32\Npcap"
call :check_and_copy_npcap "C:\Windows\SysWOW64\Npcap"
call :check_and_copy_npcap "C:\Program Files\Npcap"
call :check_and_copy_npcap "C:\Program Files (x86)\Npcap"

if %FOUND_WPCAP%==1 (
    set FOUND_RUNTIME_DLLS=1
)
goto :eof

:check_and_copy_npcap
set "NPCAP_PATH=%~1"
if exist "%NPCAP_PATH%\wpcap.dll" (
    echo ✅ Found wpcap.dll at: %NPCAP_PATH%
    copy "%NPCAP_PATH%\wpcap.dll" "%LIBS_DIR%\" >nul 2>&1
    if !errorlevel!==0 (
        echo 📦 Copied wpcap.dll to libs directory
        set FOUND_WPCAP=1
    ) else (
        echo ⚠️  Failed to copy wpcap.dll (permission issue?)
    )
)

if exist "%NPCAP_PATH%\Packet.dll" (
    echo ✅ Found Packet.dll at: %NPCAP_PATH%
    copy "%NPCAP_PATH%\Packet.dll" "%LIBS_DIR%\" >nul 2>&1
    if !errorlevel!==0 (
        echo 📦 Copied Packet.dll to libs directory
        set FOUND_PACKET=1
    ) else (
        echo ⚠️  Failed to copy Packet.dll (permission issue?)
    )
)
goto :eof
