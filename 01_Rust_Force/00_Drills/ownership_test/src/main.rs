fn main() {
    let mut s1 = String::from("Quantum");
    let r1 = &s1;
    let r2 = &mut s1;
    println!("r1 sees: {},r2 modifies: {}", r1, r2);
}
