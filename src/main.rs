
use winsafe;
fn main() {
    let mut commandline = winsafe::GetCommandLine();
    let newcmd = if commandline.starts_with("\""){
        commandline.remove(0);
        commandline.splitn(2, "\"")
    }else{
        commandline.splitn(2, " ")
    };
    println!("{}",newcmd.collect::<Vec<_>>().get(1).unwrap_or(&"error").trim_start());
}
