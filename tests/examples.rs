use sdvrp::{
    config,
    instance::{AlkaidInstance, InputFormat::*, Instance},
    solve_sdvrp,
};

fn calc_cost(solution: &[Vec<(i32, i32)>], instance: &Instance) -> i32 {
    let dist: &dyn Fn((usize, usize)) -> i32 = match instance.input_format() {
        DenseMatrix(items) => &|(a, b)| items[a][b],
        CoordList(items) => &|(a, b)| {
            (((items[a].0 - items[b].0).pow(2) + (items[a].1 - items[b].1).pow(2)) as f64).sqrt()
                as i32
        },
    };
    let mut demands = instance.demands().to_owned();
    let mut total_cost = 0;
    for route in solution {
        assert!(
            route.iter().map(|(_, x)| *x).sum::<i32>() <= instance.capacity(),
            "capacity exceeded"
        );
        for &(i, load) in route {
            demands[i as usize - 1] -= load;
        }
        route
            .windows(2)
            .for_each(|w| total_cost += dist((w[0].0 as usize, w[1].0 as usize)));
        total_cost += dist((0, route.first().unwrap().0 as usize));
        total_cost += dist((route.last().unwrap().0 as usize, 0));
    }
    assert!(demands.iter().all(|&x| x == 0), "all demands not served");
    total_cost
}

#[test]
fn test_dense_matrix() {
    #[rustfmt::skip]
    let instance = Instance::from_dense_matrix(
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
    );
    let mut config = config::Config::default();
    config.time_limit = 0.1;
    let solution = solve_sdvrp(&config, &instance);
    assert!(
        calc_cost(&solution, &instance) <= 2282842,
        "cost should be less than 2282842"
    );
    println!("{:?}", solution);
}

#[test]
fn test_coord_list() {
    #[rustfmt::skip]
    let instance = Instance::from_coord_list(
        100,
        vec![
            60, 90, 60, 90, 60, 90, 60, 90, 60, 90, 60, 90, 60, 90, 60, 90, 60, 90, 60, 90, 60, 90,
            60, 90, 60, 90, 60, 90, 60, 90, 60, 90, 60, 90, 60, 90, 60, 90, 60, 90, 60, 90, 60, 90,
            60, 90, 60, 90, 60, 90, 60, 90, 60, 90, 60, 90, 60, 90, 60, 90, 60, 90, 60, 90,
        ],
        vec![
            (    0,      0), ( 1000,      0), (  924,    383), (  707,    707), (  383,    924), (    0,   1000), ( -383,    924), 
            ( -707,    707), ( -924,    383), (-1000,      0), ( -924,   -383), ( -707,   -707), ( -383,   -924), (   -0,  -1000), 
            (  383,   -924), (  707,   -707), (  924,   -383), ( 2000,      0), ( 1848,    765), ( 1414,   1414), (  765,   1848), 
            (    0,   2000), ( -765,   1848), (-1414,   1414), (-1848,    766), (-2000,      0), (-1848,   -765), (-1414,  -1414), 
            ( -766,  -1848), (   -0,  -2000), (  765,  -1848), ( 1414,  -1414), ( 1848,   -766), ( 3000,      0), ( 2772,   1148), 
            ( 2121,   2121), ( 1148,   2772), (    0,   3000), (-1148,   2772), (-2121,   2121), (-2772,   1148), (-3000,      0), 
            (-2772,  -1148), (-2122,  -2121), (-1148,  -2771), (   -0,  -3000), ( 1148,  -2772), ( 2121,  -2122), ( 2771,  -1149), 
            ( 4000,      0), ( 3696,   1531), ( 2828,   2828), ( 1531,   3695), (    0,   4000), (-1531,   3696), (-2828,   2829), 
            (-3695,   1531), (-4000,      0), (-3696,  -1530), (-2829,  -2828), (-1531,  -3695), (   -1,  -4000), ( 1530,  -3696), 
            ( 2828,  -2829), ( 3695,  -1531),
        ],
    );
    let mut config = config::Config::default();
    config.time_limit = 5.0;
    let solution = solve_sdvrp(&config, &instance);
    assert!(
        calc_cost(&solution, &instance) <= 268475,
        "cost should be less than 268475"
    );
    println!("{:?}", solution);
}
