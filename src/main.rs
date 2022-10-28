mod contains_duplicates;
mod detect_squares;
mod intersection_of_two_array_ii;
mod maximum_subarrays;
mod merge_sorted_arrays;
mod two_sums;
mod valid_anagrams;
mod best_time_to_buy_and_sell_stocks;

fn main() {
    println!(
        "{:?}",
        intersection_of_two_array_ii::Solution::intersect(
            vec![9, 2, 2, 3, 4, 2],
            vec![9, 8, 2, 2, 4]
        )
    );
}
