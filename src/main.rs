mod calculus;

fn main() {
    let stats = calculus::Stats::create(vec![2, 3, 5, 5, 1, 2, 421, 421, 5, 6, 7, 43, 8]);
    stats.show_results();
}
