use std::io;

pub mod my_tree;
pub mod word_reader;

pub fn main() {
    let words : my_tree::Tree<String, uint> =
        word_reader::WordReader::new(io::stdin()).fold(
            my_tree::Tree::new(),
            |words, w| words.insert_or_modify(w, 1u, &mut |v| v + 1)
        );

    words.each(&mut |word: &String, n: &uint| {
        println!("{}: {}", word, *n);
    });
}
