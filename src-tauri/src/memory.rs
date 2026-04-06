// Copyright (c) 2022 Eray Erdin
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use sysinfo::{System, Pid};
use std::time::Duration;
use tauri::{Manager, AppHandle};

/// Monitor memory usage and log it periodically
pub fn start_memory_monitor(_app_handle: AppHandle) {
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let mut interval = tokio::time::interval(Duration::from_secs(300)); // Every 5 minutes
            let pid = Pid::from_u32(std::process::id());
            
            loop {
                interval.tick().await;
                
                let mut sys = System::new_all();
                sys.refresh_all();
                
                if let Some(process) = sys.process(pid) {
                    let memory_mb = process.memory() / 1024 / 1024;
                    log::info!("Memory usage: {} MB", memory_mb);
                    
                    // Warn if memory is high
                    if memory_mb > 500 {
                        log::warn!("High memory usage detected: {} MB. Consider reloading the webview.", memory_mb);
                    }
                }
            }
        });
    });
}

/// Refresh webview periodically to clear memory
/// NOTE: Disabled for WhatsApp Web as it causes disconnection issues
/// WhatsApp Web manages its own memory efficiently
#[allow(dead_code)]
pub fn start_webview_refresh(app_handle: AppHandle) {
    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            // Refresh every 4 hours
            let mut interval = tokio::time::interval(Duration::from_secs(4 * 60 * 60));
            
            loop {
                interval.tick().await;
                
                log::info!("Refreshing webview to clear memory...");
                
                // Get main window and reload it
                if let Some(window) = app_handle.get_webview_window("main") {
                    if let Err(e) = window.eval("window.location.reload()") {
                        log::error!("Failed to reload webview: {}", e);
                    } else {
                        log::info!("Webview refreshed successfully");
                    }
                }
            }
        });
    });
}

/// Clear webview cache on startup
/// NOTE: For WhatsApp Web, we don't clear localStorage to preserve session data
pub fn clear_webview_cache(app_handle: &AppHandle) {
    log::info!("Clearing temporary webview data...");
    
    // For WhatsApp Web, we only clear sessionStorage, not localStorage
    // This preserves the WhatsApp session and prevents disconnection
    if let Some(window) = app_handle.get_webview_window("main") {
        // Only clear sessionStorage, keep localStorage for WhatsApp session persistence
        let clear_storage = r#"
            try {
                sessionStorage.clear();
                console.log('Session storage cleared');
            } catch(e) {
                console.error('Error clearing session storage:', e);
            }
        "#;
        
        if let Err(e) = window.eval(clear_storage) {
            log::error!("Failed to clear session storage: {}", e);
        } else {
            log::info!("Session storage cleared successfully");
        }
    }
}
