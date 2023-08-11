use lldb::*;

fn main() {
    SBDebugger::initialize();

    let debugger = SBDebugger::create(false);
    debugger.set_asynchronous(false);
    println!("{:?}", debugger);

    if let Some(target) = debugger.create_target_simple("/usr/local/bin/servo") {
        println!("{:?}", target);

        let launchinfo = SBLaunchInfo::new();
        launchinfo.set_launch_flags(LaunchFlags::STOP_AT_ENTRY);
        match target.launch(launchinfo) {
            Ok(process) => {
                println!("{:?}", process);
                let _ = process.continue_execution();
                println!("{:?}", process);
            }
            Err(e) => println!("Uhoh: {:?}", e),
        }
    }
    SBDebugger::terminate();
}
