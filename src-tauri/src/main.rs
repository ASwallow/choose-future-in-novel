#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Serialize;
use std::fs;
use tauri::Manager;

#[derive(Serialize, Clone)]
struct StoryInfo {
    id: String,
    title: String,
    author: String,
}

#[tauri::command]
fn scan_stories(app: tauri::AppHandle) -> Result<Vec<StoryInfo>, String> {
    let mut stories = Vec::new();

    // 从打包的资源目录扫描
    if let Some(resource_path) = app.path().resource_dir().ok() {
        let stories_dir = resource_path.join("stories");
        if let Ok(entries) = fs::read_dir(&stories_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|e| e.to_str()) == Some("json") {
                    if let Some(id) = path.file_stem().and_then(|s| s.to_str()) {
                        if let Ok(content) = fs::read_to_string(&path) {
                            if let Ok(data) = serde_json::from_str::<serde_json::Value>(&content) {
                                let title = data["meta"]["title"]
                                    .as_str()
                                    .unwrap_or(id)
                                    .to_string();
                                let author = data["meta"]["author"]
                                    .as_str()
                                    .unwrap_or("未知")
                                    .to_string();
                                stories.push(StoryInfo {
                                    id: id.to_string(),
                                    title,
                                    author,
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    // 从可执行文件同目录下的 stories 文件夹扫描
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            let local_stories = exe_dir.join("stories");
            if let Ok(entries) = fs::read_dir(&local_stories) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.extension().and_then(|e| e.to_str()) == Some("json") {
                        if let Some(id) = path.file_stem().and_then(|s| s.to_str()) {
                            if !stories.iter().any(|s| s.id == id) {
                                if let Ok(content) = fs::read_to_string(&path) {
                                    if let Ok(data) =
                                        serde_json::from_str::<serde_json::Value>(&content)
                                    {
                                        let title = data["meta"]["title"]
                                            .as_str()
                                            .unwrap_or(id)
                                            .to_string();
                                        let author = data["meta"]["author"]
                                            .as_str()
                                            .unwrap_or("未知")
                                            .to_string();
                                        stories.push(StoryInfo {
                                            id: id.to_string(),
                                            title,
                                            author,
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(stories)
}

#[tauri::command]
fn load_story(app: tauri::AppHandle, story_id: String) -> Result<serde_json::Value, String> {
    // 优先从资源目录加载
    if let Some(resource_path) = app.path().resource_dir().ok() {
        let path = resource_path.join("stories").join(format!("{}.json", story_id));
        if let Ok(content) = fs::read_to_string(&path) {
            return serde_json::from_str(&content).map_err(|e| e.to_string());
        }
    }

    // 再从可执行文件同目录加载
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            let path = exe_dir
                .join("stories")
                .join(format!("{}.json", story_id));
            if let Ok(content) = fs::read_to_string(&path) {
                return serde_json::from_str(&content).map_err(|e| e.to_string());
            }
        }
    }

    Err(format!("找不到故事文件：{}", story_id))
}

#[tauri::command]
fn minimize_window(app: tauri::AppHandle) {
    if let Some(w) = app.get_webview_window("main") {
        let _ = w.minimize();
    }
}

#[tauri::command]
async fn toggle_maximize(app: tauri::AppHandle) -> Result<(), String> {
    let w = app.get_webview_window("main").ok_or("窗口不存在")?;
    let is_maximized = w.is_maximized().map_err(|e| e.to_string())?;
    if is_maximized {
        w.unmaximize().map_err(|e| e.to_string())?;
    } else {
        w.maximize().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn close_window(app: tauri::AppHandle) {
    if let Some(w) = app.get_webview_window("main") {
        let _ = w.close();
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            scan_stories,
            load_story,
            minimize_window,
            toggle_maximize,
            close_window
        ])
        .run(tauri::generate_context!())
        .expect("启动应用失败");
}
