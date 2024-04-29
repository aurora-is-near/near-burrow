use clap::CommandFactory;

#[path = "src/cli.rs"]
mod cli;

fn main() -> std::io::Result<()> {
    println!("cargo:rerun-if-changed=src/cli.rs");
    println!("cargo:rerun-if-changed=man");

    let out_dir =
        std::path::PathBuf::from(std::env::var_os("OUT_DIR").ok_or(std::io::ErrorKind::NotFound)?);

    let cmd = cli::Cli::command();

    let man = clap_mangen::Man::new(cmd);
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;

    let mut path = out_dir.ancestors().nth(3).unwrap().to_owned();
    path.push("man");
    std::fs::create_dir_all(&path).unwrap();

    std::fs::write(path.join("near-burrow.1"), buffer)?;

    Ok(())
}
