use std::thread;
use std::time::Duration;
use std::env;
use std::process::Command;
use windows::Win32::System::Power::{SetThreadExecutionState, ES_CONTINUOUS, ES_SYSTEM_REQUIRED};
use windows::Win32::UI::Input::KeyboardAndMouse::{SendInput, INPUT, INPUT_0, INPUT_MOUSE, MOUSEEVENTF_MOVE, MOUSEINPUT};
use winreg::enums::*;
use winreg::RegKey;
use windows_sys::Win32::Security::*;
use windows_sys::Win32::System::Threading::*;
use windows_sys::Win32::Foundation::*;

fn prevent_sleep() {
    unsafe {
        // 防止系统进入睡眠状态
        SetThreadExecutionState(ES_CONTINUOUS | ES_SYSTEM_REQUIRED);
    }
}

fn send_virtual_input() {
    // 构造鼠标移动事件来模拟活动
    let input = INPUT {
        r#type: INPUT_MOUSE,
        Anonymous: INPUT_0 {
            mi: MOUSEINPUT {
                dx: 0,
                dy: 1, // 小幅度移动鼠标
                mouseData: 0,
                dwFlags: MOUSEEVENTF_MOVE,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    };

    unsafe {
        // 发送虚拟输入
        SendInput(&[input], std::mem::size_of::<INPUT>() as i32);
    }
}

fn set_auto_start() -> Result<(), Box<dyn std::error::Error>> {
    let exe_path = env::current_exe()?;
    let exe_path = exe_path.to_string_lossy().to_string();
    
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let path = "Software\\Microsoft\\Windows\\CurrentVersion\\Run";
    let (key, _) = hkcu.create_subkey(path)?;
    
    key.set_value("NotLocking", &exe_path)?;
    println!("开机自启动设置成功！");
    Ok(())
}

fn run_as_admin() -> std::io::Result<()> {
    let exe_path = env::current_exe()?;
    let args: Vec<String> = env::args().skip(1).collect();
    
    Command::new("powershell")
        .args(&[
            "-Command",
            &format!(
                "Start-Process -FilePath '{}' -ArgumentList '{}' -Verb RunAs -WindowStyle Hidden",
                exe_path.to_string_lossy(),
                args.join(" ")
            ),
        ])
        .spawn()?;
    
    Ok(())
}

fn is_admin() -> bool {
    unsafe {
        let mut token_handle: HANDLE = 0;
        if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token_handle) == 0 {
            return false;
        }

        let mut elevation = TOKEN_ELEVATION { TokenIsElevated: 0 };
        let mut size = std::mem::size_of::<TOKEN_ELEVATION>() as u32;
        let result = GetTokenInformation(
            token_handle,
            TokenElevation,
            &mut elevation as *mut _ as *mut _,
            size,
            &mut size,
        );

        CloseHandle(token_handle);
        if result == 0 {
            return false;
        }

        elevation.TokenIsElevated != 0
    }
}

fn main() {
    if !is_admin() {
        println!("正在请求管理员权限...");
        if let Err(e) = run_as_admin() {
            eprintln!("错误: 无法以管理员权限运行: {}", e);
            return;
        }
        std::process::exit(0);
    }

    let args: Vec<String> = env::args().collect();
    
    if args.len() > 1 && args[1] == "--install" {
        if let Err(e) = set_auto_start() {
            eprintln!("错误: {}", e);
            return;
        }
    }
    
    loop {
        // 定期防止系统进入睡眠
        prevent_sleep();

        // 模拟用户输入，每60秒触发一次
        send_virtual_input();
        thread::sleep(Duration::from_secs(60));
    }
}
