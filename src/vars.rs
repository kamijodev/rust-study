pub mod sub_a;
pub mod sub_b;

const MAX_POINTS: u32 = 100_000;

pub fn run() {
    println!("Here is vars module");
    // sub_a::func_a();
    // sub_b::func_b();
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    let _i1 = 3;
    let _f1 = 0.1;

    println!("{}", usize::BITS);
    println!("Memory address of const is: {:p}", &MAX_POINTS);

    let i2: i64 = 1;
    let i3: i64 = 2;
    println!("Stack address of i2 is: {:p}", &i2);
    println!("Stack address of i3 is: {:p}", &i3);

    let y = 5;
    println!("Stack address of y is: {:p}", &y);
    // シャドーイング　メモリは保持されている
    let y = y + 1;
    println!("Stack address of y is: {:p}", &y);
    let y = y * 2;
    println!("Stack address of y is: {:p}", &y);
    println!("The value of y is: {}", &y);
    {
        let y = 0;
        println!("The value of y is :{}", y);
    }
    println!("The value of y is :{}", y);

    // タプル
    let t1 = (500, 6.4, "dummy");
    let (x, y, z) = t1;
    println!("The value of t1 is: {} {} {}", t1.0, t1.1, t1.2);

    let mut t2 = ((0, 1), (2, 3));
    // ポインタの値の取得
    let ((ref mut x1_ptr, ref mut y1_ptr), _) = t2;

    // 参照外し
    *x1_ptr = 5;
    *y1_ptr = -5;
    // 複雑なデータ型は{:?}で表示出来る
    println!("{:?}", t2);

    // 配列
    let a1 = [1, 2, 3, 4, 5];
    let a2 = [0; 10];
    println!("{:?} {:?} {} {}", a1, a2, a1[2], a1[3]);

    // 文字列スライスはスタックではなく静的領域に格納される
    // スタックには16バイトのデータが入る8バイトが静的領域のメモリの場所残る8バイトが要素数(len)
    let s1 = "helloこんにちは挨拶";
    let s2 = "hello";
    println!("Stack address of s1 is:{:p}", &s1);
    println!("Stack address of s2 is:{:p}", &s2);
    println!("Static memory address of s1: {:?}", s1.as_ptr());
    println!("Static memory address of s2: {:?}", s2.as_ptr());
    println!("Len of s1: {}", s1.len());
    println!("Len of s1: {}", s1.chars().count());
    println!("Len of s2: {}", s2.len());

    // string型は文字列スライスと違ってヒープ領域を使用する
    // stackは24バイト使用する
    // スライスとの違いはcapacity(cap)が存在する
    // ptr, len, cap
    let mut s1 = String::from("hello");
    let mut s2 = String::from("helloworld");
    println!("Stack address of s1 is: {:p}", &s1);
    println!("Stack address of s2 is: {:p}", &s2);
    println!("Heap memory address of s1: {:p}", s1.as_ptr());
    println!("Heap memory address of s2: {:p}", s2.as_ptr());
    println!("Len of s1 is: {}", s1.len());
    println!("Len of s2 is: {}", s2.len());
    println!("Capacity of s1 is: {}", s1.capacity());
    println!("Capacity of s2 is: {}", s2.capacity());
    s1.push_str("_new1");
    s2.push_str("_new2");
    println!("{} {}", s1, s2);

    // 所有権者のみがメモリの解放ができる
}
