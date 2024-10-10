use std::convert::TryInto;

fn main() {
    // 使用 into() 进行安全的转换
    let x: i32 = 5;
    let y: i64 = x.into(); // i32 转换为 i64
    println!("y: {}", y);

    // 使用 try_into() 进行可能失败的转换
    let a: i32 = 300;
    let b: u8 = match a.try_into() {
        Ok(val) => val,
        Err(_) => {
            println!("转换失败: a 的值超出了 u8 的范围");
            return;
        }
    };
    println!("b: {}", b);
}
