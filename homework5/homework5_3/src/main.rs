// 实现一个打印面积的函数，它接收一个计算面积的类型作为参数，
//比如 圆形、三角形、正方形

pub trait Calc{
    fn area(&self)->i32;
}

struct Triangle{
    length:i32
}

impl Calc for Triangle{
    fn area(&self)->i32{
        40
    }
}

struct Square{
    length:i32
}

impl Calc for Square{
    fn area(&self)->i32{
        50
    }
}

pub fn do_something(item: &impl Calc){
    println!("{}", item.area());
}

fn main() {
    let shape = Square{ length: 10};
    do_something(&shape);
}
