# Npcap Runtime Dependencies

This directory should contain the Npcap runtime DLLs needed for the application to work on machines without Npcap installed.

## Required Files:
- `wpcap.dll` - WinPcap API library
- `Packet.dll` - Low-level packet access library  

## How to obtain these files:

### Option 1: Copy from your system (if Npcap is installed)
Copy the DLLs from your Npcap installation:
- `C:\Windows\System32\Npcap\wpcap.dll`
- `C:\Windows\System32\Npcap\Packet.dll`

### Option 2: Extract from Npcap installer
1. Download Npcap installer from https://npcap.com/dist/
2. Extract the installer using 7-Zip or similar
3. Copy the DLLs from the extracted files

### Option 3: Use the build script
The build script will automatically try to copy these from your system installation.

## License Notice:
These DLLs are part of Npcap and subject to their license terms.
Make sure to comply with Npcap's licensing requirements when distributing.
