// Importing a Rust function
@external("host-exported", "echo")
declare function echo(): void;

export function add(a: i32, b: i32): i32 {
    return a + b;
}


export function start(): i32 {
    echo();
    return 0;
}
