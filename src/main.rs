use std::thread;
use std::time::Duration;
use windows::Win32::System::Power::{SetThreadExecutionState, ES_CONTINUOUS, ES_SYSTEM_REQUIRED};
use windows::Win32::UI::Input::KeyboardAndMouse::{SendInput, INPUT, INPUT_0, INPUT_MOUSE, MOUSEEVENTF_MOVE, MOUSEINPUT};

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

fn main() {
    loop {
        // 定期防止系统进入睡眠
        prevent_sleep();

        // 模拟用户输入，每60秒触发一次
        send_virtual_input();
        thread::sleep(Duration::from_secs(60));
    }
}
