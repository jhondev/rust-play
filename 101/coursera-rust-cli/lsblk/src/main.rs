use clap::Parser;

#[derive(Parser)]
#[command(version = "0.0.1", author = "Jhonattan", about = "lsblk in Rust")]

struct Opts {
    #[clap(subcommand)]
    cmd: Command,
}

#[derive(Parser)]
enum Command {
    #[clap(name = "info", about = "get info about a device")]
    Info(InfoOpts),
}

#[derive(Parser)]
struct InfoOpts {
    #[clap(help = "device to get info about")]
    device: String,
}

fn main() {
    let opts = Opts::parse();
    match opts.cmd {
        Command::Info(info) => {
            let device = info.device;
            let output = std::process::Command::new("lsblk")
                .arg("-o")
                .arg("NAME,SIZE,TYPE,MOUNTPOINT")
                .arg(device)
                .output()
                .expect("failed to execute process");
            println!("{:?}", output.stdout);
        }
    }
}
