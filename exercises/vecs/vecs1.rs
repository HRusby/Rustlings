// vecs1.rs
// Your task is to create a `Vec` which holds the exact same elements
// as in the array `a`.
// Make me compile and pass the test!
// Execute `rustlings hint vecs1` or use the `hint` watch subcommand for a hint.

const a : [i32; 4] = [10, 20, 30, 40];

// Three Options. 
fn array_and_vec_1() -> Vec<i32> {
    // 1: Define In-line
    vec![a[0], a[1], a[2], a[3]]
}

fn array_and_vec_2() -> Vec<i32> {
    // 2: Define empty and push elements
    let mut v = Vec::new();
    v.push(a[0]);
    v.push(a[1]);
    v.push(a[2]);
    v.push(a[3]);
    v
}

fn array_and_vec_3() -> Vec<i32> {
    // 3: Define from a slice (array can be treated as a slice)
    a.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let v = array_and_vec_1();
        assert_eq!(a, v[..]);
        let v = array_and_vec_2();
        assert_eq!(a, v[..]);
        let v = array_and_vec_3();
        assert_eq!(a, v[..]);
    }
}
