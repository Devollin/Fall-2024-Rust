mod student;
mod major;

fn main() {
    let s = student::Student::new("Neo", "CS");
    println!("{:?}", s);
    println!("{:?}", major::Major::ComputerScience);
}
