mod lib;
use lib::*;
fn main() {
    let map = Map::<&str>::from(vec![
        ("Como estas", "Bien, gracias"),
        ("Quien eres", "Me llamo Alex")
    ]);
    //run(input, train(map, &utils::CONFIG), &lib::utils::CONFIG);
    // Split_whitespace and collect into vector
    println!("{:#?}", utils::chunks(vec![2343, 325, 3421, 4565, 1244, 2465]));
}