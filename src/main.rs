#[cxx::bridge(namespace = "alkaidsd")]
mod ffi {
    unsafe extern "C++" {
        include!("sdvrp/src/AlkaidSDVRP.h");

        unsafe fn solve_sdvrp(
            random_seed: u32,
            time_limit: f64,
            blink_rate: f64,
            inter_operators: Vec<&str>,
            intra_operators: Vec<&str>,
            acceptance_rule_type: &str,
            lahc_length: i32,
            sa_initial_temperature: f64,
            sa_decay: f64,
            ruin_method_type: &str,
            sisrs_average_customers: i32,
            sisrs_max_length: i32,
            sisrs_split_rate: f64,
            sisrs_preserved_probability: f64,
            random_ruin_sizes: Vec<i32>,
            sorters: Vec<&str>,
            sorter_values: Vec<f64>,

            capacity: i32,
            demands: Vec<i32>,
            input_format: &str,
            distance_matrix: Vec<i32>,
            coord_list_x: Vec<i32>,
            coord_list_y: Vec<i32>,
        ) -> Vec<i32>;
    }
}

fn split_results(results: Vec<i32>) -> Vec<Vec<(i32, i32)>> {
    let mut routes = vec![];
    enum SplitState {
        WaitingRoute,
        WaitingNode,
        WaitingLoad(i32),
    }
    let mut state = SplitState::WaitingRoute;
    for i in results {
        match (state, i) {
            (SplitState::WaitingRoute, 0) => {
                routes.push(vec![]);
                state = SplitState::WaitingNode;
            }
            (SplitState::WaitingNode, 0) => {
                state = SplitState::WaitingRoute;
            }
            (SplitState::WaitingNode, node) => {
                state = SplitState::WaitingLoad(node);
            }
            (SplitState::WaitingLoad(node), load) => {
                routes.last_mut().unwrap().push((node, load));
                state = SplitState::WaitingNode;
            }
            (SplitState::WaitingRoute, _) => unreachable!(),
        }
    }
    return routes;
}

fn main() {
    let cfg = unsafe {
        ffi::solve_sdvrp(
            42,
            5.0,
            0.021,
            vec![
                "Relocate",
                "Swap<2, 0>",
                "Swap<2, 1>",
                "Swap<2, 2>",
                "Cross",
                "SwapStar",
                "SdSwapStar",
            ],
            vec!["Exchange", "OrOpt<1>"],
            "LAHC",
            83,
            100.0,
            0.95,
            "SISRs",
            36,
            8,
            0.740,
            0.096,
            vec![1, 2, 3, 4, 5],
            vec!["random", "demand", "far", "close"],
            vec![0.078, 0.225, 0.942, 0.120],
            100,
            vec![60, 90, 60, 90, 60, 90, 60, 90],
            "COORD_LIST",
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            vec![0, 1000, 0, -1000, -0, 2000, 0, -2000, -0],
            vec![0, 0, 1000, 0, -1000, 0, 2000, 0, -2000],
        )
    };
    println!("{:?}", split_results(cfg));
}
