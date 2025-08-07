#include "AlkaidSDVRP.h"

void UpdateInterOperators(alkaidsd::AlkaidConfig &config, rust::Vec<rust::Str> &inter_operators)
{
    for (const auto arg : inter_operators)
    {
        if (arg == "Swap<2, 0>")
        {
            config.inter_operators.push_back(std::make_unique<alkaidsd::inter_operator::Swap<2, 0>>());
        }
        else if (arg == "Swap<2, 1>")
        {
            config.inter_operators.push_back(std::make_unique<alkaidsd::inter_operator::Swap<2, 1>>());
        }
        else if (arg == "Swap<2, 2>")
        {
            config.inter_operators.push_back(std::make_unique<alkaidsd::inter_operator::Swap<2, 2>>());
        }
        else if (arg == "Relocate")
        {
            config.inter_operators.push_back(std::make_unique<alkaidsd::inter_operator::Relocate>());
        }
        else if (arg == "SwapStar")
        {
            config.inter_operators.push_back(std::make_unique<alkaidsd::inter_operator::SwapStar>());
        }
        else if (arg == "Cross")
        {
            config.inter_operators.push_back(std::make_unique<alkaidsd::inter_operator::Cross>());
        }
        else if (arg == "SdSwapStar")
        {
            config.inter_operators.push_back(std::make_unique<alkaidsd::inter_operator::SdSwapStar>());
        }
        else if (arg == "SdSwapOneOne")
        {
            config.inter_operators.push_back(std::make_unique<alkaidsd::inter_operator::SdSwapOneOne>());
        }
        else if (arg == "SdSwapTwoOne")
        {
            config.inter_operators.push_back(std::make_unique<alkaidsd::inter_operator::SdSwapTwoOne>());
        }
        else
        {
            throw std::invalid_argument("Invalid inter operator.");
        }
    }
}

void UpdateIntraOperators(alkaidsd::AlkaidConfig &config, rust::Vec<rust::Str> &intra_operators)
{
    for (const auto arg : intra_operators)
    {
        if (arg == "Exchange")
        {
            config.intra_operators.push_back(std::make_unique<alkaidsd::intra_operator::Exchange>());
        }
        else if (arg == "OrOpt<1>")
        {
            config.intra_operators.push_back(std::make_unique<alkaidsd::intra_operator::OrOpt<1>>());
        }
        else if (arg == "OrOpt<2>")
        {
            config.intra_operators.push_back(std::make_unique<alkaidsd::intra_operator::OrOpt<2>>());
        }
        else if (arg == "OrOpt<3>")
        {
            config.intra_operators.push_back(std::make_unique<alkaidsd::intra_operator::OrOpt<3>>());
        }
        else
        {
            throw std::invalid_argument("Invalid intra operator.");
        }
    }
}

void UpdateAcceptanceRule(alkaidsd::AlkaidConfig &config, rust::Str acceptance_rule_type, int lahc_length,
                          double sa_initial_temperature,
                          double sa_decay)
{
    if (acceptance_rule_type == "LAHC")
    {
        config.acceptance_rule = ([lahc_length]()
                                  { return std::make_unique<alkaidsd::acceptance_rule::LateAcceptanceHillClimbing>(lahc_length); });
    }
    else if (acceptance_rule_type == "SA")
    {
        config.acceptance_rule = ([sa_initial_temperature, sa_decay]()
                                  { return std::make_unique<alkaidsd::acceptance_rule::SimulatedAnnealing>(sa_initial_temperature,
                                                                                                           sa_decay); });
    }
    else if (acceptance_rule_type == "HCWE")
    {
        config.acceptance_rule = ([]()
                                  { return std::make_unique<alkaidsd::acceptance_rule::HillClimbingWithEqual>(); });
    }
    else
    {
        config.acceptance_rule = ([]()
                                  { return std::make_unique<alkaidsd::acceptance_rule::HillClimbing>(); });
    }
}

void UpdateRuinMethod(alkaidsd::AlkaidConfig &config, rust::Str ruin_method_type, int sisrs_average_customers,
                      int sisrs_max_length,
                      double sisrs_split_rate,
                      double sisrs_preserved_probability,
                      rust::Vec<int> &random_ruin_sizes)
{
    if (ruin_method_type == "SISRs")
    {
        config.ruin_method = std::make_unique<alkaidsd::ruin_method::SisrsRuin>(sisrs_average_customers, sisrs_max_length,
                                                                                sisrs_split_rate, sisrs_preserved_probability);
    }
    else
    {
        auto num_perturb_customers = std::vector<int>{};
        num_perturb_customers.insert(num_perturb_customers.end(), random_ruin_sizes.begin(), random_ruin_sizes.end());
        config.ruin_method = std::make_unique<alkaidsd::ruin_method::RandomRuin>(num_perturb_customers);
    }
}

void UpdateSorter(alkaidsd::AlkaidConfig &config, rust::Vec<rust::Str> &sorters,
                  rust::Vec<double> &sorter_values)
{
    for (size_t i = 0; i < sorters.size(); ++i)
    {
        if (sorters[i] == "random")
        {
            config.sorter.AddSortFunction(std::make_unique<alkaidsd::sorter::SortByRandom>(), sorter_values[i]);
        }
        else if (sorters[i] == "demand")
        {
            config.sorter.AddSortFunction(std::make_unique<alkaidsd::sorter::SortByDemand>(), sorter_values[i]);
        }
        else if (sorters[i] == "far")
        {
            config.sorter.AddSortFunction(std::make_unique<alkaidsd::sorter::SortByFar>(), sorter_values[i]);
        }
        else if (sorters[i] == "close")
        {
            config.sorter.AddSortFunction(std::make_unique<alkaidsd::sorter::SortByClose>(), sorter_values[i]);
        }
        else
        {
            throw std::invalid_argument("Invalid sort function.");
        }
    }
}

class SimpleListener : public alkaidsd::Listener
{
public:
    void OnStart() override { start_time_ = std::chrono::system_clock::now(); }
    void OnUpdated([[maybe_unused]] const alkaidsd::AlkaidSolution &solution, int objective) override
    {
        auto elapsed_time = std::chrono::duration_cast<std::chrono::duration<double>>(
            std::chrono::system_clock::now() - start_time_);
        std::cout << "Update at " << elapsed_time.count() << "s: " << objective << std::endl;
    }
    void OnEnd([[maybe_unused]] const alkaidsd::AlkaidSolution &solution, int objective) override
    {
        auto elapsed_time = std::chrono::duration_cast<std::chrono::duration<double>>(
            std::chrono::system_clock::now() - start_time_);
        std::cout << "End at " << elapsed_time.count() << "s: " << objective << std::endl;
    }

private:
    std::chrono::system_clock::time_point start_time_;
};

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
        rust::Vec<int> coord_list_y)
    {
        AlkaidConfig config;
        config.random_seed = random_seed;
        config.time_limit = time_limit;
        config.blink_rate = blink_rate;
        UpdateInterOperators(config, inter_operators);
        UpdateIntraOperators(config, intra_operators);
        UpdateAcceptanceRule(config, acceptance_rule_type, lahc_length, sa_initial_temperature, sa_decay);
        UpdateRuinMethod(config, ruin_method_type, sisrs_average_customers, sisrs_max_length,
                         sisrs_split_rate, sisrs_preserved_probability, random_ruin_sizes);
        UpdateSorter(config, sorters, sorter_values);
        config.listener = std::make_unique<SimpleListener>();

        alkaidsd::Instance instance;
        instance.num_customers = demands.size() + 1; // +1 for depot
        instance.capacity = capacity;
        instance.demands.resize(instance.num_customers);
        for (alkaidsd::Node i = 1; i < instance.num_customers; ++i)
        {
            instance.demands[i] = demands[i - 1];
        }
        instance.distance_matrix.resize(instance.num_customers);
        if (input_format == "DENSE_MATRIX")
        {
            for (alkaidsd::Node i = 0; i < instance.num_customers; ++i)
            {
                instance.distance_matrix[i].resize(instance.num_customers);
                for (alkaidsd::Node j = 0; j < instance.num_customers; ++j)
                {
                    instance.distance_matrix[i][j] = distance_matrix[i * instance.num_customers + j];
                }
            }
        }
        else if (input_format == "COORD_LIST")
        {
            for (alkaidsd::Node i = 0; i < instance.num_customers; ++i)
            {
                instance.distance_matrix[i].resize(instance.num_customers);
                for (alkaidsd::Node j = 0; j < instance.num_customers; ++j)
                {
                    auto x1 = coord_list_x[i];
                    auto y1 = coord_list_y[i];
                    auto x2 = coord_list_x[j];
                    auto y2 = coord_list_y[j];
                    instance.distance_matrix[i][j] = lround(hypot(x1 - x2, y1 - y2));
                }
            }
        }
        else
        {
            throw std::invalid_argument("Invalid input_format.");
        }

        auto distance_matrix_optimizer = alkaidsd::DistanceMatrixOptimizer(instance.distance_matrix);
        alkaidsd::AlkaidSolver solver;
        auto solution = solver.Solve(config, instance);
        distance_matrix_optimizer.Restore(solution);

        rust::Vec<int> result;
        for (Node node_index : solution.NodeIndices())
        {
            if (!solution.Predecessor(node_index))
            {
                result.push_back(0);
                while (node_index)
                {
                    Node customer = solution.Customer(node_index);
                    result.push_back(customer);
                    result.push_back(solution.Load(node_index));
                    node_index = solution.Successor(node_index);
                }
                result.push_back(0);
            }
        }
        return result;
    }
}