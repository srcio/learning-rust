fn main() {
    // 向量 Vector
    // 哈希表 HashMap
    // 哈希集合 HashSet

    vec1()
}

// 向量 Vector（Vec），在别的语言中可能叫做切片 slice
// - 相同元素的集合
// - 长度可变
// - 内存在堆上
fn vec1() {
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    // v.push("hello");// 会报错
    println!("v: {:?}", v);
    println!("v[0]: {}", v[0]);

    // 通过 vec! 宏创建向量
    let mut v2 = vec![1, 2, 3, 4, 5];
    println!("v2: {:?}", v2);
    let i = v2.remove(0);
    println!("i: {:?}", i);
    println!("v2: {:?}", v2);

    if v2.contains(&5) {
        println!("v2 contains 5");
    }

    for item in v2 {
        println!("item: {}", item);
    }
}

