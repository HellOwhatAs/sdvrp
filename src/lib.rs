pub mod config;
pub mod instance;

#[cxx::bridge(namespace = "alkaidsd")]
pub mod ffi {
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

pub fn solve_sdvrp<T: config::AlkaidConfig, T2: instance::AlkaidInstance>(
    config: &T,
    instance: &T2,
) -> Vec<Vec<(i32, i32)>> {
    let result = unsafe {
        ffi::solve_sdvrp(
            config.random_seed(),
            config.time_limit(),
            config.blink_rate(),
            config
                .inter_operators()
                .iter()
                .map(|e| e.to_str())
                .collect(),
            config
                .intra_operators()
                .iter()
                .map(|e| e.to_str())
                .collect(),
            config.acceptance_rule_type().to_str(),
            config.acceptance_rule_type().to_length(),
            config.acceptance_rule_type().to_initial_temperature(),
            config.acceptance_rule_type().to_decay(),
            config.ruin_method_type().to_str(),
            config.ruin_method_type().to_average_customers(),
            config.ruin_method_type().to_max_length(),
            config.ruin_method_type().to_split_rate(),
            config.ruin_method_type().to_preserved_probability(),
            config.ruin_method_type().to_random_ruin_sizes(),
            config.sorters().iter().map(|(e, _)| e.to_str()).collect(),
            config.sorters().iter().map(|(_, e)| *e).collect(),
            instance.capacity(),
            instance.demands().to_vec(),
            instance.input_format().to_str(),
            instance.input_format().to_dense_matrix(),
            instance.input_format().to_coord_list_x(),
            instance.input_format().to_coord_list_y(),
        )
    };
    return split_results(result);
}
