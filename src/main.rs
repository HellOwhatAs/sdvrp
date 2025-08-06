#[cxx::bridge(namespace = "alkaidsd")]
mod ffi {
    unsafe extern "C++" {
        include!("sdvrp/src/AlkaidSDVRP.h");

        type AlkaidSolution;
        type AlkaidConfig;
        type Instance;
        type AlkaidSolver;

        fn test_func() -> i32;
    }
}

fn main() {
    let _sol: ffi::AlkaidSolution;
    let _cfg: ffi::AlkaidConfig;
    let _ins: ffi::Instance;
    let _solver: ffi::AlkaidSolver;
    println!("{:?}", ffi::test_func());
}
