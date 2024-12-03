use blockchainlib::*;


fn main() {
    let mut block = Block::new(1, 5, vec![0;32], 1, "Genesis Block!".to_owned());

    println!("{:?}" ,&block);

    let h = block.hash();

    println!("{:?}", &h);
    
    block.hash = h;

    println!("{:?}" ,&block);

}