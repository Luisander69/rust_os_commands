use std::process::Command;
use std::env;

fn executecmd(cmd: &str) -> String{
    
let res = if cfg!(target_os = "windows") {
        let temp = "/C ".to_owned();
        let fullcmd = temp + cmd;
        let cmds: Vec<&str> = fullcmd.split(" ").collect();
        Command::new("cmd").arg(&cmds[0]).arg(&cmds[1]).output().unwrap()
} else {
        let temp = "-c ".to_owned();
        let fullcmd = temp + cmd;
        let cmds: Vec<&str> = fullcmd.split(" ").collect();
        Command::new("sh").arg(&cmds[0]).arg(&cmds[1]).output().unwrap()
};

        let stdout = String::from_utf8_lossy(res.stdout.as_slice());
        let stderr = String::from_utf8_lossy(res.stderr.as_slice());

        if stdout.len() > 0{
            return stdout.to_string();
        }else{
            return stderr.to_string();
        }

}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let res = executecmd(&args[1]);
        println!("{}", res);
    }else{
        println!("[+]Usage {} command", args[0]);
    }
}
