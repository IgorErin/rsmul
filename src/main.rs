use std::path::{PathBuf, Path};

use clap::Parser;
use sprs::io::{read_matrix_market, write_matrix_market};

use std::env;

#[derive(Parser, Debug, Clone, Copy)]
struct Cli {
    #[arg(short, long, default_value_t = 2u32)]
    count: u32,

    // todo default to option none
    #[arg(short, long, default_value_t = 3u32)]
    to: u32,
}

fn string_of_pathbuf(pathbuf: PathBuf) -> String {
    pathbuf
        .into_os_string()
        .into_string()
        .expect("buf to string conversion failed")
}

fn get_current_dir() -> PathBuf {
    env::current_dir().expect("unable to take current dir")
}

fn mk_result_path(src: &Path, count: u32, dest_dir: &Path) -> PathBuf {
    let name = take_name(src);
    let result_name = format!("{}{}", name, count);

    let mut result: PathBuf = dest_dir.into();
    result.push(result_name);

    result
}

fn run_path(src_path: &Path, count: u32, dest_path: &Path) {
    let mtx: rsmul::TriMat =
        read_matrix_market(src_path).unwrap_or_else(|e| panic!("reading error: {e}"));

    let result: rsmul::PatternMat = rsmul::muln(count, mtx);

    let result = write_matrix_market(dest_path, &result);

    result.unwrap_or_else(|e| panic!("writing error: {e}"));
}

fn mk_src_glob() -> String {
    let mut s = get_current_dir();
    s.push("*.mtx");

    string_of_pathbuf(s)
}

fn take_name(path: &Path) -> String {
    let name_os = path.file_name().expect("file name doesnt exist");

    name_os.to_os_string().into_string().unwrap()
}

mod check {
    use super::*;

    pub fn glob(glob : Result<PathBuf, glob::GlobError>) -> PathBuf {
        glob.unwrap_or_else(|x| panic!("glob error: {x}"))
    }

    pub fn args(args : Cli) {
        fn end_with(m: String) -> ! {
            panic!("{m}")
        }

        match () {
            () if args.count < 1 => {
                let m = format!("count = {} < 1. Cannot powered in {}", args.count, args.count);
                end_with(m);
            },
            () if args.count > args.to => {
                let m = format!("count = {} > {} = to. Lower bound greater then upper. Strange...", args.count, args.to);
                end_with(m);
            }
            () => ()
        };
    }
}

fn main() {
    let args = Cli::parse();

    check::args(args);

    let src_glog = mk_src_glob();

    println!("src glob: {}\n", src_glog);
    println!("writing to the current directory\n");

    for src_path in glob::glob(&src_glog).expect("glob erorr") {
        let src_path = check::glob(src_path);

        for count in args.count .. args.to {
            println!("start: {} for {count}", take_name(&src_path));

            let dest_path = mk_result_path(&src_path, count, &get_current_dir());

            // if exists then nothing to do
            if Path::exists(&dest_path) {
                println!("exist: {}", take_name(&dest_path));
                continue;
            }

            run_path(&src_path, count, &dest_path);

            println!("end with: {:?}\n", take_name(&dest_path));
        }
    }
}
