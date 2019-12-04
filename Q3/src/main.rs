//Q3
#[derive(Debug)]
struct Cacher<T>
where T: Fn(i32) -> i32, {
    power : T,
}
impl <T> Cacher<T>
where T: Fn(i32) -> i32, {
    fn Calculation(power:T) -> Cacher<T> {
            Cacher{ power,}
        }
    }

fn main () {
let x = Cacher {
    power: |x| x*x
};
println!("{}",(x.power)(4));
}