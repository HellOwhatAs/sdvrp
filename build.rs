fn main() {
    let dst = cmake::Config::new("Alkaid-SDVRP")
        .define("EXTRA_SOURCE_FILE", "src/AlkaidSDVRP.cpp")
        .build();

    println!(
        "cargo:rustc-link-search=native={}/lib/AlkaidSD-1.0",
        dst.display()
    );
    println!("cargo:rustc-link-lib=static=AlkaidSD");
    println!("cargo:rerun-if-changed=Alkaid-SDVRP");
    println!("cargo:rerun-if-changed=src/AlkaidSDVRP.cpp");
}
