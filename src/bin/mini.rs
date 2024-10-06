fn main() {
    let l = [1, 2, 3, 4, 5];
    let is_nature = l.iter().any(|&x| x == 3);
    println!("{}", is_nature);
    println!("{:?}", l.iter().all(|x| x >= &0));
    let l: [i32; 0] = [];
    println!("{:?}", l.iter().any(|x| x >= &0));
    println!("{:?}", l.iter().all(|x| x >= &0));
}
