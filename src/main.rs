fn main() {
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


