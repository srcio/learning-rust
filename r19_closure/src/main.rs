fn main() {
    closure1();
}

// closure：闭包（内联函数）
// 你可以把它看作一个匿名函数，在一个函数内创建一个可以立即调用的函数
// - 声明时使用 || 替代 () 将输入参数括起来
// - 函数体定界符 ({}) 对于单个表达式是可选的，其他情况必须加上
// - 有能力捕获外部环境的变量

fn closure1() {
    let add = |a, b| {
        println!("add called");
        a + b
    };
    let x = add(1, 2);
    println!("x: {:?}", x);

    let double = |x| x * 2;
    let y = double(3);
    println!("y: {:?}", y);

    let v = 100;
    let add100 = |x| { x + v };
    let z = add100(50);

    println!("z: {:?}", z);
}

// 捕获
// 闭包本质上很灵活，可以在没有类型标注的情况下运行。
// 可以移动（move），可以借用（borrow）。
// 可以通过一下方式捕获：
// 1 引用：&T
// 2 可变引用：&mut T
// 3 通过值：T