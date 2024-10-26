// Generics
struct V2<T> {
    x: T,
    y: T,
}

fn with_x<T>(new_value: T, vec: V2<T>) -> V2<T> {
    V2{ x: new_value, y: vec.y}
}

fn main() {
    let iv = V2 { x: 1, y: 2 };
    println!("iv: x:{} y:{}", iv.x, iv.y);
    
    let fv = V2 { x: 1.2, y: 3.4 };
    println!("fv: x:{} y:{}", fv.x, fv.y);

    let new_vec = with_x(10, iv);
    println!("New vector: x:{}, y:{}", new_vec.x, new_vec.y);
}