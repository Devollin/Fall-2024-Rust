fn borrow_to_mut_watchout() {
    let mut word = "UT".to_string(); 
    fn update(word: &mut String) {
        word.push_str("RGV");
    }
    update(&mut word);
    println!("{word}")
}

fn borrowing_doesnot_move_ownership() {
    let word: String = "UTRGV".to_string();
    let borrow_word: &String = &word;
    let borrowed_word: &String = &word;
    println!("{} == {} == {}", word, borrow_word, borrowed_word);
}

fn main() {
    let x: i32 = 5;
    let y: &i32 = &x;
    println!("{}", y);
    println!("{:p} == {:p}", y, &x);
    borrowing_doesnot_move_ownership();
    borrow_to_mut_watchout();
}