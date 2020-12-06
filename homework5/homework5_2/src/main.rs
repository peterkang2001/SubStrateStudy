// 实现一个函数，为u32类型的集合求和，参数类型为&[u32],返回类型为Option<u32>,溢出时返回None

fn iterator_sum(list: &[u32])-> u32{
    let mut sum:u32 = 0;
    for item in list.iter(){
        sum = sum + *item as u32;
    }
   sum
}

fn main() {
    let number_list = vec![1, 2, 3, 4];
    let sum = iterator_sum(&number_list);
    println!("{}", sum);
}
