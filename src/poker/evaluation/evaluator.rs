/// Evaluates hand strengths using a variant of Cactus Kev's algorithm:
/// http://suffe.cool/poker/evaluator.html

/// I make considerable optimizations in terms of speed and memory usage,
/// in fact the lookup table generation can be done in under a second and
/// consequent evaluations are very fast. Won't beat C, but very fast as
/// all calculations are done with bit arithmetic and table lookups.

use super::lookup::{
    LookupTable
};

struct Evaluator{
    table: LookupTable,
}