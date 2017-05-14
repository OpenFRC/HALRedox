use std::env;
use std::process::Command;
fn main() {
    Command::new("make").arg("-C").arg("cpp/wrapper").arg("arm").output();

    if env::var("TARGET").unwrap() == "arm-unknown-linux-gnueabi" {
        for lib in ["NiFpga",
                    "NiFpgaLv"].iter()
        {
            println!("cargo:rustc-link-lib=dylib={}", lib);
        }
        println!("cargo:rustc-link-search=native=lib/ni-libraries");
        println!("cargo:rustc-link-search=native=lib/ni-wrapper");

        println!("cargo:rustc-link-lib=static=ni_wrapper.a")
    }
}
