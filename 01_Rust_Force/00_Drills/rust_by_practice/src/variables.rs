#![allow(warnings)]
#[test]

// 修复下面代码的错误并尽可能少的修改
fn test_3_1() {
    let x: i32 = 5; // 未初始化，但被使用
    let y: i32; // 未初始化，也未被使用
    println!("x is equal to {}", x);
}

#[test]

// 完形填空，让代码编译
fn test_3_2() {
    let mut x = 1;
    x += 2;

    println!("x = {}", x);
}

#[test]

// 修复下面代码的错误并使用尽可能少的改变
fn test_3_3() {
    let x: i32 = 10;
    let y: i32 = 2;
    {
        let y: i32 = 5;
        println!("x 的值是 {}, y 的值是 {}", x, y);
    }
    println!("x 的值是 {}, y 的值是 {}", x, y);
}

#[test]
// 修复错误
fn test_3_4() {
    define_x();
}

fn define_x() {
    let x = "hello";
    println!("{},world", x)
}

#[test]

// 只允许修改 `assert_eq!` 来让 `println!` 工作(在终端输出 `42`)
fn test_3_5() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12);
    }

    assert_eq!(x, 5);

    let x = 42;
    println!("{}", x); // 输出 "42".
}

#[test]
fn test_3_6() {
    let mut x: i32 = 1;
    x = 7;
    // 遮蔽且再次绑定
    let x = x;

    let y = 4;
    // 遮蔽
    let y = "I can also be bound to text!";
}

#[test]

fn test_3_7() {
    let _x = 1;
}

#[test]

// 修复下面代码的错误并尽可能少的修改
fn test_3_8() {
    let (mut x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);
}

#[test]

fn test_3_9() {
    let (x, y);
    (x, ..) = (3, 4);
    [.., y] = [1, 2];
    // 填空，让代码工作
    assert_eq!([x, y], [3, 2]);
}
