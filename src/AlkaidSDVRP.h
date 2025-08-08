#pragma once

#include <alkaidsd/solver.h>
#include <alkaidsd/distance_matrix_optimizer.h>
#include <chrono>
#include <iostream>
#include <fstream>
#include <cmath>
#include "rust/cxx.h"
#include "sdvrp/src/lib.rs.h"

namespace alkaidsd
{
    rust::Vec<int> solve_sdvrp(
        uint32_t random_seed,
        double time_limit,
        double blink_rate,
        rust::Vec<rust::Str> inter_operators,
        rust::Vec<rust::Str> intra_operators,
        rust::Str acceptance_rule_type,
        int lahc_length,
        double sa_initial_temperature,
        double sa_decay,
        rust::Str ruin_method_type,
        int sisrs_average_customers,
        int sisrs_max_length,
        double sisrs_split_rate,
        double sisrs_preserved_probability,
        rust::Vec<int> random_ruin_sizes,
        rust::Vec<rust::Str> sorters,
        rust::Vec<double> sorter_values,

        int capacity,
        rust::Vec<int> demands,
        rust::Str input_format,
        rust::Vec<int> distance_matrix,
        rust::Vec<int> coord_list_x,
        rust::Vec<int> coord_list_y);
}