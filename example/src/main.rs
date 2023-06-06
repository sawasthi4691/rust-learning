fn main() {
    let a: i32 = 2;
    let result: i32 = stack_only(a);
    dbg!(result);
}

fn stack_only(b: i32) -> i32 {
    let c: i32 = 3;
    return b + c + stack_and_heap();
}

fn stack_and_heap() -> i32 {
    let d: i32 = 5;
    let e: Box<i32> = Box::new(7);
    return d + *e;
}
