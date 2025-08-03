fn main() {

    bits_acculate();
   // float_compare();
    //return
    shadowing();
}

//变量遮掩（shadowing) 2.1
fn shadowing(){
    let x = 5;
    //在main函数作用域对之前的x进行遮掩
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x); //6 *2 =12
    }
    println!("The value of shadowing is: {}", x); // 6
}
//变量遮掩的用途：变量遮蔽的用处在于，如果你在某个作用域内无需再使用之前的变量
// （在被遮蔽后，无法再访问到之前的同名变量），就可以重复的使用变量名字，而不用
// 绞尽脑汁去想更多的名字

// 2025/8/3 2:52 基本数据类型
// 数据类型：基本类型和复合类型。 基本类型：数值类型、布尔类型、字符类型、字符串类型、单元类型。
// 数值类型：i8、i16、i32、i64、i128、isize、u8、u16、u32、u64、u128、usize
// 字符串类型：String、&str
// 布尔类型：true、false
// fn float_compare() {
//     let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
//     let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);
//
//     println!("abc (f32)");
//     println!("   0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
//     println!("         0.3: {:x}", (abc.2).to_bits());
//     println!();
//
//     println!("xyz (f64)");
//     println!("   0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
//     println!("         0.3: {:x}", (xyz.2).to_bits());
//     println!();
//
//     assert!(abc.0 + abc.1 == abc.2);
//     assert!(xyz.0 + xyz.1 == xyz.2);
// }

fn bits_acculate() {
    // 无符号8位整数，二进制为00000010
    let a: u8 = 2; // 也可以写 let a: u8 = 0b_0000_0010;

    // 二进制为00000011
    let b: u8 = 3;

    // {:08b}：左高右低输出二进制01，不足8位则高位补0
    println!("a value is        {:08b}", a);

    println!("b value is        {:08b}", b);

    println!("(a & b) value is  {:08b}", a & b);

    println!("(a | b) value is  {:08b}", a | b);

    println!("(a ^ b) value is  {:08b}", a ^ b);

    println!("(!b) value is     {:08b}", !b);

    println!("(a << b) value is {:08b}", a << b);

    println!("(a >> b) value is {:08b}", a >> b);

    let mut a = a;
    // 注意这些计算符除了!之外都可以加上=进行赋值 (因为!=要用来判断不等于)
    a <<= b;
    println!("(a << b) value is {:08b}", a);
}

