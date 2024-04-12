use std::path::PathBuf;

use clap::Parser;
use sprs::io::{read_matrix_market, write_matrix_market};

#[derive(Parser, Debug)]
struct Cli {
    #[arg(short, long, default_value_t = 1u32)]
    count: u32,

    #[arg(short, long)]
    src_path: std::path::PathBuf,

    #[arg(short, long)]
    dest_path: std::path::PathBuf,
}

fn mk_result_path(src: &PathBuf, count: u32, dest_dir: &PathBuf) -> PathBuf {
    let name_os = src.file_name().expect("file name doesnt exist");
    let name_str = name_os.to_os_string().into_string().unwrap();

    let mut result = dest_dir
        .clone()
        .into_os_string()
        .into_string()
        .expect("buf to string conversiont failed");

    result.push_str(&name_str);
    result.push_str(&format!("{count}"));

    PathBuf::from(result)
}

fn run_path(src_path: PathBuf, count: u32, dest_path: &PathBuf) {
    let mtx: rsmul::TriMat =
        read_matrix_market(src_path).unwrap_or_else(|e| panic!("reading error: {e}"));

    let result: rsmul::PatternMat = rsmul::muln(count, mtx);

    let result = write_matrix_market(dest_path, &result);

    result.unwrap_or_else(|e| panic!("writing error: {e}"));
}

fn main() {
    let args = Cli::parse();

    let src_glog = {
        let mut s = args
            .src_path
            .into_os_string()
            .into_string()
            .expect("Path to string convertation failed");

        s.push_str("/*.mtx");

        s
    };

    println!("src glob: {:?}", src_glog);

    for src_path in glob::glob(&src_glog).expect("glob erorr") {
        println!("src path: {:?}", src_path);

        let src_path = src_path.expect("glob error...");

        let dest_path = mk_result_path(&src_path, args.count, &args.dest_path);

        println!("dest path: {:?}", &dest_path);

        run_path(src_path, args.count, &dest_path);
    }
}
