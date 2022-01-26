use std::io::Write;
use std::{env, fs::File, path::Path};

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();

    let _ = dotenv::dotenv();
    println!("cargo:rerun-if-changed=.env");

    let yt_id = env::var("YT_API_CLIENT_ID").expect("Missing env variable YT_API_CLIENT_ID");
    let yt_secret =
        env::var("YT_API_CLIENT_SECRET").expect("Missing env variable YT_API_CLIENT_SECRET");
    println!("cargo:rerun-if-env-changed=YT_API_CLIENT_ID");
    println!("cargo:rerun-if-env-changed=YT_API_CLIENT_SECRET");

    let dest_path = Path::new(&out_dir).join("yt_credentials.rs");
    let mut file = File::create(dest_path).unwrap();
    writeln!(
        &mut file,
        "const fn yt_client_id() -> &'static str {{ {:?} }}",
        yt_id
    )
    .unwrap();
    writeln!(
        &mut file,
        "const fn yt_client_secret() -> &'static str {{ {:?} }}",
        yt_secret
    )
    .unwrap();
}
