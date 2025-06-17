use std::process::Command;

fn main() {
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
