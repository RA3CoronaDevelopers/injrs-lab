use injrs::evelate_windows::*;
use injrs::inject_windows::*;
use injrs::process_windows::*;

const USAGE_HELP: &str = "
USAGE:
injrs PROCESS_PATH/PID [Libraies...]

EXAMPLES:
1. Inject test.dll to process Calc
    $ injrs Calc test.dll

2. Inject test.dll and demo.dll to process with PID: 1888
    $ injrs 1888 test.dll demo.dll
";

fn main() {
    println!("Welcome to have injrs. A library injector written by Rust.");
    // load args
    let mut args = std::env::args();
    if args.len() < 2 {
        println!("{}", USAGE_HELP);
        return;
    }
    let pid_or_name = args.nth(1).expect("must give a process name or pid");
    let dlls: Vec<String> = std::env::args().skip(2).collect();
    if dlls.len() == 0 {
        println!("You at least give one file to inject");
        return;
    }

    let process: Process;
    match pid_or_name.parse::<u32>() {
        Ok(pid) => {
            // process pid
            match Process::from_pid(pid) {
                None => {
                    println!("can't find process with pid: {}", pid);
                    return;
                }
                Some(p) => process = p,
            }
        }
        Err(_) => {
            let program_path = pid_or_name.as_str();

            // 直接寻找程序路径并执行

            use std::os::windows::process::CommandExt;
            use std::process::{Command, Stdio};
            use std::thread;
            // 创建Command对象
            let mut command = Command::new(program_path);

            // 如果你的程序需要传递参数，可以使用args方法，比如：
            // command.args(&["arg1", "arg2"]);

            // 设置启动子进程时的标志
            command.creation_flags(0x00000008); // DETACHED_PROCESS = 0x00000008，表示创建一个不与控制台相关联的进程

            // 设置子进程的输出重定向（如果需要的话）
            command.stdout(Stdio::null());
            command.stderr(Stdio::null());

            // 启动子进程
            let mut child = command.spawn().expect("无法启动子进程");

            // 获取子进程的PID
            let pid = child.id() as u32;

            // 在一个新线程中等待子进程完成（如果不关心子进程的退出状态，也可以不等待）
            thread::spawn(move || {
                let _ = child.wait(); // 等待子进程完成，并忽略结果
            });

            println!("子进程的PID：{}", pid);
            match Process::from_pid(pid) {
                None => {
                    println!("can't find process with pid: {}", pid);
                    return;
                }
                Some(p) => process = p,
            }
        }
    }

    let _ = evelate_privileges();

    for i in dlls {
        print!("{} => ", i);
        match process.inject(i.as_str()) {
            Err(e) => {
                println!("error: {}", e);
            }
            Ok(_) => {
                println!("success");
            }
        }
    }
}
