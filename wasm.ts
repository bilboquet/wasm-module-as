// Importing a Rust function
@external("host-exported", "hello")
declare function hello(): void;

export function add(a: i32, b: i32): i32 {
    return a + b;
}


export function start(): i32 {
    hello();
    return 0;
}

export function greet(name: string): string {
    return "Hello " + name + "!";
}
