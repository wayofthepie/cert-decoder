use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    cert_path: String,
}

fn main() {
    let args = Opt::from_args();
    println!("{:#?}", args);
}
