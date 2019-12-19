use std::process::Command;
use which::which;

fn main() {
    println!("gen protoc");
    let grpc_rust_plugin_path = which("grpc_rust_plugin")
        .expect("fail get extension path `grpc_rust_plugin`")
        .to_str()
        .expect("fail path string")
        .to_string();

    let output = Command::new("protoc")
        .arg("--rust_out=./src/")
        .arg("--grpc_out=./src/")
        .arg(format!("--plugin=protoc-gen-grpc={}", grpc_rust_plugin_path).as_str())
        .arg("./proto/iost.proto")
        .output()
        .expect("fail command protoc");
    println!("{}", output.status);
    println!("{}", String::from_utf8_lossy(&output.stdout));
    println!("{}", String::from_utf8_lossy(&output.stderr));
}