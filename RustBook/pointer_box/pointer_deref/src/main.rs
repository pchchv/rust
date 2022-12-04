// fn main() {
//     let x = 5;
//     let y = &x;

//     assert_eq!(5, x);
//     assert_eq!(5, *y);
// }

fn main() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
