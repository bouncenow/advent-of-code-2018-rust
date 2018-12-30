use raoc::ch2::*;

fn main() {
    println!("{}", checksum_for_ids_in_file("ch2.txt"));
    for common_part in common_parts_of_closest_strings("ch2.txt") {
        println!("{}", common_part);
    }
}
