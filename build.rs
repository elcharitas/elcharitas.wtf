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
    // Rebuild if any component changes.
    println!("cargo:rerun-if-changed=src/components");
    let status = Command::new("npx")
        .args([
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
