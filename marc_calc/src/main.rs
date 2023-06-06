use std::io;

fn main() {
    //println!("Hello, world!");

    //println!("result {} Kg", calculate_weight_on_mars(50.00));
    //all variables in rust are immutable by default. That just means that once we assign a value to them, it can never be changed.
    //We have to explicitly declare a variable as immutable in order to be able to mutated.
    //To make it mutable we need to add keyword mut.
    let mut input: String = String::new();
    println!("Enter the weight (kg)");
    io::stdin().read_line(&mut input).unwrap(); // unwrap function will error out file there is an isse with read line function.
    //println!("Input {}", input);

    let weight: f32 = input.trim().parse().unwrap(); // unwrap function will error out file there is an isse with read line function.
    //dbg!(weight); // to see debug value
   // println!("weight {}", weight);
    //println!("converted weight to float {}", weight);
    let mut mars_weight = calculate_weight_on_mars(weight);
    println!("result {} Kg", mars_weight);
    mars_weight = mars_weight * 1000.0;
    println!("result {} Grams", mars_weight);

    //concepts of reference and borrowing.
    /*let mut val = String::new();
    val = val + "hello";

    let a1 = &val;
    let b1 = &val;
    //give error  cannot borrow `val` as mutable because it is also borrowed as immutable.
    //let mut c1 = &mut val;
    some(a1);
    some(b1);
    //some_mut(c1);*/
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight) / 9.81 * 3.711
}

fn some(res: &String) {
    println!("res {}", res);
}

fn some_mut(res: &mut String) {
    println!("res {}", res);

}
