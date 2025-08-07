mod config;
mod instance;

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

fn solve_sdvrp<T: config::AlkaidConfig, T2: instance::AlkaidInstance>(
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

#[rustfmt::skip]
fn dense_matrix_example() -> instance::Instance {
    instance::Instance::from_dense_matrix(
        100,
        vec![60, 90, 60, 90, 60, 90, 60, 90],
        vec![
            vec![0, 100000, 100000, 100000, 100000, 200000, 200000, 200000, 200000],
            vec![100000, 0, 141421, 200000, 141421, 100000, 223607, 300000, 223607],
            vec![100000, 141421, 0, 141421, 200000, 223607, 100000, 223607, 300000],
            vec![100000, 200000, 141421, 0, 141421, 300000, 223607, 100000, 223607],
            vec![100000, 141421, 200000, 141421, 0, 223607, 300000, 223607, 100000],
            vec![200000, 100000, 223607, 300000, 223607, 0, 282843, 400000, 282843],
            vec![200000, 223607, 100000, 223607, 300000, 282843, 0, 282843, 400000],
            vec![200000, 300000, 223607, 100000, 223607, 400000, 282843, 0, 282843],
            vec![200000, 223607, 300000, 223607, 100000, 282843, 400000, 282843, 0],
        ],
    )
}

#[rustfmt::skip]
fn coord_list_example() -> instance::Instance {
    instance::Instance::from_coord_list(
        100,
        vec![
            60, 90, 60, 90, 60, 90, 60, 90, 60, 90, 60, 90, 60, 90, 60, 90, 60, 90, 60, 90, 60, 90,
            60, 90, 60, 90, 60, 90, 60, 90, 60, 90, 60, 90, 60, 90, 60, 90, 60, 90, 60, 90, 60, 90,
            60, 90, 60, 90, 60, 90, 60, 90, 60, 90, 60, 90, 60, 90, 60, 90, 60, 90, 60, 90,
        ],
        vec![
            (    0,      0),
            ( 1000,      0),
            (  924,    383),
            (  707,    707),
            (  383,    924),
            (    0,   1000),
            ( -383,    924),
            ( -707,    707),
            ( -924,    383),
            (-1000,      0),
            ( -924,   -383),
            ( -707,   -707),
            ( -383,   -924),
            (   -0,  -1000),
            (  383,   -924),
            (  707,   -707),
            (  924,   -383),
            ( 2000,      0),
            ( 1848,    765),
            ( 1414,   1414),
            (  765,   1848),
            (    0,   2000),
            ( -765,   1848),
            (-1414,   1414),
            (-1848,    766),
            (-2000,      0),
            (-1848,   -765),
            (-1414,  -1414),
            ( -766,  -1848),
            (   -0,  -2000),
            (  765,  -1848),
            ( 1414,  -1414),
            ( 1848,   -766),
            ( 3000,      0),
            ( 2772,   1148),
            ( 2121,   2121),
            ( 1148,   2772),
            (    0,   3000),
            (-1148,   2772),
            (-2121,   2121),
            (-2772,   1148),
            (-3000,      0),
            (-2772,  -1148),
            (-2122,  -2121),
            (-1148,  -2771),
            (   -0,  -3000),
            ( 1148,  -2772),
            ( 2121,  -2122),
            ( 2771,  -1149),
            ( 4000,      0),
            ( 3696,   1531),
            ( 2828,   2828),
            ( 1531,   3695),
            (    0,   4000),
            (-1531,   3696),
            (-2828,   2829),
            (-3695,   1531),
            (-4000,      0),
            (-3696,  -1530),
            (-2829,  -2828),
            (-1531,  -3695),
            (   -1,  -4000),
            ( 1530,  -3696),
            ( 2828,  -2829),
            ( 3695,  -1531),
        ],
    )
}

fn main() {
    let mut config = config::Config::default();
    config.time_limit = 5.0;
    println!("{:?}", solve_sdvrp(&config, &dense_matrix_example()));
    config.time_limit = 10.0;
    println!("{:?}", solve_sdvrp(&config, &coord_list_example()));
}
