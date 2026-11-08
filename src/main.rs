#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tao::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wry::WebViewBuilder;
use std::path::PathBuf;
use std::fs;

fn find_index() -> Option<PathBuf> {
    // Ищем index.html рядом с exe, потом в текущей папке, потом в dist/
    let candidates = [
        PathBuf::from("index.html"),
        PathBuf::from("dist/index.html"),
        PathBuf::from("../index.html"),
    ];
    for p in &candidates {
        if p.exists() {
            return Some(fs::canonicalize(p).ok()?);
        }
    }
    // Рядом с exe
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            let p = dir.join("index.html");
            if p.exists() {
                return Some(p);
            }
            let p2 = dir.join("dist").join("index.html");
            if p2.exists() {
                return Some(p2);
            }
        }
    }
    None
}

fn main() {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("Windows 34")
        .with_inner_size(tao::dpi::LogicalSize::new(1024.0, 720.0))
        .with_min_inner_size(tao::dpi::LogicalSize::new(640.0, 480.0))
        .build(&event_loop)
        .expect("Не удалось создать окно");

    let index = match find_index() {
        Some(p) => p,
        None => {
            eprintln!("❌ index.html не найден! Положи index.html рядом с exe или в папку dist/");
            std::process::exit(1);
        }
    };

    println!("📂 Открываю: {}", index.display());

    let webview = WebViewBuilder::new(&window)
        .with_url(format!("file://{}", index.display()))
        .with_devtools(true)
        .with_initialization_script(r#"
            // Разрешаем localStorage (обычно работает и так)
            try { localStorage.setItem('_test', '1'); localStorage.removeItem('_test'); } catch(e) {}
        "#)
        .build()
        .expect("Не удалось создать WebView");

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            _ => (),
        }
        let _ = &webview;
    });
}
