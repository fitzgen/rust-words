use std::io;

mod my_tree;
mod word_reader;

fn main() {
    let words : my_tree::Tree<String, uint> =
        word_reader::WordReader::new(io::stdin()).fold(
            my_tree::Tree::new(),
            |words, w| words.insert_or_modify(w, 1u, |v| v + 1)
        );

    words.each(&mut |word: &String, n: &uint| {
        println!("{}: {}", word, *n);
    });
}
