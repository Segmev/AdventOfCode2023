mod part1;
mod part2;
mod utils;

fn main() {
    let file_content = utils::get_file_content(&utils::get_input_path());

    part1::start(&file_content);
}
