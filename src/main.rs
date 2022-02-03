mod lib;
use lib::*;
fn main() {
    let map = Map::<&str>::from(vec![
        ("Como estas", "Bien, gracias"),
        ("Quien eres", "Me llamo Alex")
    ]);
    //run(input, train(map, &utils::CONFIG), &lib::utils::CONFIG);
    train(map);
}