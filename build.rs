use std::process::Command;

fn main() {
    // rerun if yarn.lock changes
    println!("cargo:rerun-if-changed=yarn.lock");
    let status = Command::new("npx")
        .args(["yarn", "install"])
        .status()
        .expect("Failed to build your CSS");

    if !status.success() {
        panic!("Could not install your dependencies.");
    }
    // Rebuild if any component changes or global.css changes
    println!("cargo:rerun-if-changed=src/components");
    println!("cargo:rerun-if-changed=global.css");
    let status = Command::new("npx")
        .args([
            "npx",
            "tailwindcss",
            "-i",
            "./global.css",
            "-o",
            "./public/styles.css",
            "--minify",
        ])
        .status()
        .expect("Failed to build your CSS");

    if !status.success() {
        panic!("Build Script execution failed");
    }
}
