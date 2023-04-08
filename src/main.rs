mod utils;
use utils::{count, count_duplicates, dig_pow, in_array, likes, reverse_words, string_to_number};

fn main() {
    let vect = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, -11, -12, -13, -14, -15].to_vec();
    // println!("{}", string_to_number("-7"));
    // println!("{:?}", digitize(1531216));
    // println!("{:?}", count_positives_sum_negatives(vect));
    // println!("{:?}", likes(&["Peter", "said", "hamza"]));
    // println!("{:?}", alphabet_position("The sunset sets at twelve clock"));
    // println!("{:?}", count_duplicates("aabBcde"));
    // println!(
    //     "{:?}",
    //     in_array(
    //         &["arp", "arp", "live", "strong"],
    //         &["arp", "live", "strong"]
    //     )
    // );
    println!("{:?}", count("aabb"))
}
