use crate::components::block::Block;

mod components;

fn main() {
    let rows: Vec<String> = vec![".00", "00", "0"]
        .iter()
        .map(|e| (*e).to_owned())
        .collect();
    let block = Block::new('A', rows).unwrap();
    println!("{:?}", block);
}
