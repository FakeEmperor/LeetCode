use std::{cell::RefCell, rc::Rc};

mod best_time_to_buy_and_sell_stocks;
mod contains_duplicates;
mod detect_squares;
mod first_unique_character_in_a_strings;
mod implement_queue_using_stacks;
mod integer_to_romans;
mod intersection_of_two_array_ii;
mod maximum_subarrays;
mod merge_sorted_arrays;
mod pascals_triangles;
mod path_sum;
mod ransom_notes;
mod remove_duplicates_from_sorted_list;
mod reshape_the_matrixs;
mod roman_to_integers;
mod search_a_2_d_matrixs;
mod two_sums;
mod valid_anagrams;
mod valid_parentheses;
mod valid_sudokus;
mod validate_binary_search_trees;
mod two_sum_iv_input_is_a_bst;

fn main() {
    let a = Some(Box::<_>::new(32));
    let b = a.as_ref();
    println!(
        "{:?}",
        path_sum::Solution::has_path_sum(
            Some(Rc::new(RefCell::new(path_sum::TreeNode {
                val: 1,
                right: None,
                left: Some(Rc::new(RefCell::new(path_sum::TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(path_sum::TreeNode {
                        val: 3,
                        left: Some(Rc::new(RefCell::new(path_sum::TreeNode {
                            val: 4,
                            left: Some(Rc::new(RefCell::new(path_sum::TreeNode {
                                val: 5,
                                left: None,
                                right: None
                            }))),
                            right: None
                        }))),
                        right: None
                    }))),
                    right: None
                }))),
            }))),
            1
        )
    );
}
