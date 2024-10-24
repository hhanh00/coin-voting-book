use mdbook::MDBook;

pub fn main() {
    let root_dir = ".";

    MDBook::load(root_dir).unwrap()
        .build()
        .expect("Book generation failed");
}
