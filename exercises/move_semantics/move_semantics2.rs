// move_semantics2.rs
// Make me compile without changing line 13 or moving line 10!
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand for a hint.


fn main() {
    // For Options 1/2
    let vec0 = Vec::new();
    // For Option 3
    // let mut vec0 = Vec::new();
    // OPTION 1:
    let vec0cp = vec0.to_vec();
    let mut vec1 = fill_vec_1(vec0cp);
    // OPTION 2:
    // let mut vec2 = fill_vec_2(&vec0);
    // OPTION 3:
    // fill_vec_3(&mut vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    // Not Required in Option 3:
    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec_1(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
// Option 2: Make method borrow it's arguments then make a copy inside the method
fn fill_vec_2(vec: &Vec<i32>) -> Vec<i32> {
    let mut vec = vec.to_vec();

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}

// Option 3: Make fill_vec mutably borrow a reference to its argument
fn fill_vec_3(vec: &mut Vec<i32>) {
    vec.push(22);
    vec.push(44);
    vec.push(66);
}
