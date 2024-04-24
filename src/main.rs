use std::path::{PathBuf, Path};

use clap::Parser;
use sprs::io::{read_matrix_market, write_matrix_market};

use std::env;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(short, long, default_value_t = 1u32)]
    count: u32,
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

fn mk_result_path(src: &PathBuf, count: u32, dest_dir: &PathBuf) -> PathBuf {
    let name = take_name(src);
    let result_name = format!("{}{}", name, count + 1);

    let mut result = dest_dir.clone();
    result.push(result_name);

    result
}

fn run_path(src_path: PathBuf, count: u32, dest_path: &PathBuf) {
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

fn check_glob(glob : Result<PathBuf, glob::GlobError>) -> PathBuf {
    glob.unwrap_or_else(|x| panic!("glob error: {x}"))
}

fn main() {
    let args = Cli::parse();

    let src_glog = mk_src_glob();
    println!("src glob: {:?}\n", src_glog);

    for src_path in glob::glob(&src_glog).expect("glob erorr") {
        let src_path = check_glob(src_path);
        println!("start: {:?}", take_name(&src_path));

        let dest_path = mk_result_path(&src_path, args.count, &get_current_dir());
        // if exists than nothing to do
        if Path::exists(&dest_path) {
            println!("exist: {:?}", dest_path);
            continue;
        }

        run_path(src_path, args.count, &dest_path);

        println!("end with: {:?}", take_name(&dest_path));
    }
}
