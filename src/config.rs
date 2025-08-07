#![allow(dead_code)]

/// inter-route operators to be used by the algorithm.
#[derive(Clone, Copy)]
pub enum InterOperators {
    Swap20,
    Swap21,
    Swap22,
    Relocate,
    SwapStar,
    Cross,
    SdSwapStar,
    SdSwapOneOne,
    SdSwapTwoOne,
}

impl InterOperators {
    pub fn to_str(&self) -> &'static str {
        match self {
            InterOperators::Swap20 => "Swap<2, 0>",
            InterOperators::Swap21 => "Swap<2, 1>",
            InterOperators::Swap22 => "Swap<2, 2>",
            InterOperators::Relocate => "Relocate",
            InterOperators::SwapStar => "SwapStar",
            InterOperators::Cross => "Cross",
            InterOperators::SdSwapStar => "SdSwapStar",
            InterOperators::SdSwapOneOne => "SdSwapOneOne",
            InterOperators::SdSwapTwoOne => "SdSwapTwoOne",
        }
    }
}

/// intra-route operators to be used by the algorithm.
#[derive(Clone, Copy)]
pub enum IntraOperators {
    Exchange,
    OrOpt1,
    OrOpt2,
    OrOpt3,
}

impl IntraOperators {
    pub fn to_str(&self) -> &'static str {
        match self {
            IntraOperators::Exchange => "Exchange",
            IntraOperators::OrOpt1 => "OrOpt<1>",
            IntraOperators::OrOpt2 => "OrOpt<2>",
            IntraOperators::OrOpt3 => "OrOpt<3>",
        }
    }
}

/// the type and arguments of acceptance rule to be used by the algorithm.
#[derive(Clone, Copy)]
pub enum AcceptanceRuleType {
    /// Hill Climbing
    HC,
    /// Hill Climbing With Equal
    HCWE,
    /// Late Acceptance Hill Climbing
    /// - length: the length of the history list.
    LAHC(i32),
    /// Simulated Annealing
    /// - initial_temperature: the initial temperature.
    /// - decay: the decay rate.
    SA(f64, f64),
}

impl Default for AcceptanceRuleType {
    fn default() -> Self {
        AcceptanceRuleType::LAHC(83)
    }
}

impl AcceptanceRuleType {
    pub fn to_str(&self) -> &'static str {
        match self {
            AcceptanceRuleType::HC => "HC",
            AcceptanceRuleType::HCWE => "HCWE",
            AcceptanceRuleType::LAHC(_) => "LAHC",
            AcceptanceRuleType::SA(_, _) => "SA",
        }
    }

    pub fn to_length(&self) -> i32 {
        match self {
            AcceptanceRuleType::LAHC(length) => *length,
            _ => 0,
        }
    }

    pub fn to_initial_temperature(&self) -> f64 {
        match self {
            AcceptanceRuleType::SA(initial_temperature, _) => *initial_temperature,
            _ => 0.0,
        }
    }

    pub fn to_decay(&self) -> f64 {
        match self {
            AcceptanceRuleType::SA(_, decay) => *decay,
            _ => 0.0,
        }
    }
}

/// the type and arguments of ruin method to be used by the algorithm.
#[derive(Clone)]
pub enum RuinMethodType {
    /// Slack Induction by String Removals
    /// - average_customers: the average number of customers to be removed.
    /// - max_length: the maximum length of the route to be removed.
    /// - split_rate: the split rate.
    /// - preserved_probability: the probability of preserving a node.
    SISRs(i32, i32, f64, f64),
    /// Random: Random Ruin
    /// - args are integer values representing all possible ruin sizes.
    Random(Vec<i32>),
}

impl Default for RuinMethodType {
    fn default() -> Self {
        RuinMethodType::SISRs(36, 8, 0.740, 0.096)
    }
}

impl RuinMethodType {
    pub fn to_str(&self) -> &'static str {
        match self {
            RuinMethodType::SISRs(_, _, _, _) => "SISRs",
            RuinMethodType::Random(_) => "Random",
        }
    }

    pub fn to_average_customers(&self) -> i32 {
        match self {
            RuinMethodType::SISRs(average_customers, _, _, _) => *average_customers,
            _ => 0,
        }
    }

    pub fn to_max_length(&self) -> i32 {
        match self {
            RuinMethodType::SISRs(_, max_length, _, _) => *max_length,
            _ => 0,
        }
    }

    pub fn to_split_rate(&self) -> f64 {
        match self {
            RuinMethodType::SISRs(_, _, split_rate, _) => *split_rate,
            _ => 0.0,
        }
    }

    pub fn to_preserved_probability(&self) -> f64 {
        match self {
            RuinMethodType::SISRs(_, _, _, preserved_probability) => *preserved_probability,
            _ => 0.0,
        }
    }

    pub fn to_random_ruin_sizes(&self) -> Vec<i32> {
        match self {
            RuinMethodType::Random(sizes) => sizes.clone(),
            _ => vec![],
        }
    }
}

/// sorter to be used by the perturbation process.
#[derive(Clone, Copy)]
pub enum Sorter {
    /// randomly shuffles customers.
    Random,
    /// sorts customers based on their demand in descending order.
    Demand,
    /// sorts customers based on their distance to the depot in descending order.
    Far,
    /// sorts customers based on their distance to the depot in increasing order.
    Close,
}

impl Sorter {
    pub fn to_str(&self) -> &'static str {
        match self {
            Sorter::Random => "random",
            Sorter::Demand => "demand",
            Sorter::Far => "far",
            Sorter::Close => "close",
        }
    }
}

pub trait AlkaidConfig {
    /// the seed value for the random number generator used by the algorithm.
    fn random_seed(&self) -> u32;
    /// the maximum time limit (in seconds) for the algorithm to run.
    fn time_limit(&self) -> f64;
    /// the blink rate for the SplitReinsertion process.
    fn blink_rate(&self) -> f64;
    /// the list of inter-route operators to be used by the algorithm.
    fn inter_operators(&self) -> &[InterOperators];
    /// the list of intra-route operators to be used by the algorithm.
    fn intra_operators(&self) -> &[IntraOperators];
    /// the type and arguments of acceptance rule to be used by the algorithm.
    fn acceptance_rule_type(&self) -> AcceptanceRuleType;
    /// the type and arguments of ruin method to be used by the algorithm.
    fn ruin_method_type(&self) -> RuinMethodType;
    /// the list of sorters to be used by the perturbation process.
    fn sorters(&self) -> &[(Sorter, f64)];
}

pub struct Config {
    /// the seed value for the random number generator used by the algorithm.
    pub random_seed: u32,
    /// the maximum time limit (in seconds) for the algorithm to run.
    pub time_limit: f64,
    /// the blink rate for the SplitReinsertion process.
    pub blink_rate: f64,
    /// the list of inter-route operators to be used by the algorithm.
    pub inter_operators: Vec<InterOperators>,
    /// the list of intra-route operators to be used by the algorithm.
    pub intra_operators: Vec<IntraOperators>,
    /// the type and arguments of acceptance rule to be used by the algorithm.
    pub acceptance_rule_type: AcceptanceRuleType,
    /// the type and arguments of ruin method to be used by the algorithm.
    pub ruin_method_type: RuinMethodType,
    /// the list of sorters to be used by the perturbation process.
    pub sorters: Vec<(Sorter, f64)>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            random_seed: 42,
            time_limit: 20.0,
            blink_rate: 0.021,
            inter_operators: vec![
                InterOperators::Relocate,
                InterOperators::Swap20,
                InterOperators::Swap21,
                InterOperators::Swap22,
                InterOperators::Cross,
                InterOperators::SwapStar,
                InterOperators::SdSwapStar,
            ],
            intra_operators: vec![IntraOperators::Exchange, IntraOperators::OrOpt1],
            acceptance_rule_type: AcceptanceRuleType::default(),
            ruin_method_type: RuinMethodType::default(),
            sorters: vec![
                (Sorter::Random, 0.078),
                (Sorter::Demand, 0.225),
                (Sorter::Far, 0.942),
                (Sorter::Close, 0.120),
            ],
        }
    }
}

impl AlkaidConfig for Config {
    fn random_seed(&self) -> u32 {
        self.random_seed
    }

    fn time_limit(&self) -> f64 {
        self.time_limit
    }

    fn blink_rate(&self) -> f64 {
        self.blink_rate
    }

    fn inter_operators(&self) -> &[InterOperators] {
        &self.inter_operators
    }

    fn intra_operators(&self) -> &[IntraOperators] {
        &self.intra_operators
    }

    fn acceptance_rule_type(&self) -> AcceptanceRuleType {
        self.acceptance_rule_type
    }

    fn ruin_method_type(&self) -> RuinMethodType {
        self.ruin_method_type.clone()
    }

    fn sorters(&self) -> &[(Sorter, f64)] {
        &self.sorters
    }
}
