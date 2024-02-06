const THERMAL_HEAT_EXPANSIVITY: f32= 237.27;


fn main() {
    let x = 5;
    let x = x+1;

    println!("Thermal heat constant : {THERMAL_HEAT_EXPANSIVITY}");

    {
        let x = x +2;
        println!("The value of x in the x inner {x}");
    }

    println!("The value of x in outer {x}");

    let heart_eyed_cat = '😻';

    println!("Emoji: {heart_eyed_cat}");

    // Tuple
    let tup:(i32,f64,u8) = (200,9.7,9);


    let five_hundred = tup.0;

    let six_point_four = tup.1;

    let one = tup.2;

    println!("tupple: {tup}");

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

}
