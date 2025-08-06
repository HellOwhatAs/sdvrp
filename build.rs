use glob::glob;

fn main() {
    let srcs = glob("Alkaid-SDVRP/src/**/*.cpp")
        .unwrap()
        .into_iter()
        .map(|e| e.unwrap().display().to_string())
        .chain(["src/AlkaidSDVRP.cpp".to_owned()].into_iter())
        .collect::<Vec<_>>();

    cxx_build::bridge("src/main.rs")
        .files(&srcs)
        .include("Alkaid-SDVRP/include")
        .include("src/")
        .std("c++17")
        .compile("Alkaid-SDVRP");

    println!("cargo:rerun-if-changed=Alkaid-SDVRP");
}
