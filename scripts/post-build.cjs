const { execSync } = require('child_process');
const path = require('path');
const fs = require('fs');

/**
 * Cross-platform post-build script for bundling dependencies
 * This ensures the app works on target machines without manual installation
 */

console.log('🔧 Running post-build processing...');

const platform = process.platform;
const projectRoot = path.join(__dirname, '..');

console.log(`📱 Platform: ${platform}`);
console.log(`📁 Project root: ${projectRoot}`);

try {
    if (platform === 'darwin') {
        // macOS: Bundle libpcap
        console.log('🍎 Running macOS post-build script...');
        
        const postBuildScript = path.join(projectRoot, 'post-build-macos.sh');
        
        if (fs.existsSync(postBuildScript)) {
            // Make script executable
            execSync(`chmod +x "${postBuildScript}"`, { cwd: projectRoot });
            
            // Execute the script
            execSync(`bash "${postBuildScript}"`, { 
                cwd: projectRoot,
                stdio: 'inherit' 
            });
            
            console.log('✅ macOS post-build completed successfully');
        } else {
            console.log('⚠️  macOS post-build script not found, skipping...');
        }
        
    } else if (platform === 'win32') {
        // Windows: Bundle Npcap DLLs
        console.log('🪟 Running Windows post-build script...');
        
        const postBuildScript = path.join(projectRoot, 'post-build-windows.bat');
        
        if (fs.existsSync(postBuildScript)) {
            // Execute the Windows batch script
            execSync(`"${postBuildScript}"`, { 
                cwd: projectRoot,
                stdio: 'inherit',
                shell: true
            });
            
            console.log('✅ Windows post-build completed successfully');
        } else {
            console.log('⚠️  Windows post-build script not found, skipping...');
            
            // Fallback: Basic verification
            const releasePath = path.join(projectRoot, 'src-tauri/target/release');
            const binaryPath = path.join(releasePath, 'InnoMonitor.exe');
            
            if (fs.existsSync(binaryPath)) {
                console.log('✅ Windows binary found:', binaryPath);
                
                // Check for Npcap DLLs in system
                const npcapPaths = [
                    'C:\\Windows\\System32\\Npcap\\wpcap.dll',
                    'C:\\Windows\\SysWOW64\\Npcap\\wpcap.dll',
                    'C:\\Program Files\\Npcap\\wpcap.dll',
                    'C:\\Program Files (x86)\\Npcap\\wpcap.dll'
                ];
                
                let npcapFound = false;
                for (const npcapPath of npcapPaths) {
                    if (fs.existsSync(npcapPath)) {
                        console.log('✅ Found Npcap at:', npcapPath);
                        npcapFound = true;
                        break;
                    }
                }
                
                if (!npcapFound) {
                    console.log('⚠️  Npcap not found - app may fail on machines without Npcap');
                    console.log('💡 Install Npcap from: https://npcap.com/');
                }
            } else {
                console.log('❌ Windows binary not found - build may have failed');
            }
        }
        
    } else {
        console.log(`⚠️  No specific post-build steps for platform: ${platform}`);
    }
    
} catch (error) {
    console.error('❌ Post-build script failed:', error.message);
    process.exit(1);
}

console.log('🎉 Post-build processing complete!');
