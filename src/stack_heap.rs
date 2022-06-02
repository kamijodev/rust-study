pub fn run() {
    // rustのstackの上限は8MB
    let a1: [u8; 7_000_000] = [1; 7_000_000];
    // stack overflowが発生する
    // let a1: [u8; 9_000_000] = [1; 9_000_000];

    // vector型は動的に要素の数を変更できる
    // string型と同じ ptr len cap
    let mut v1 = vec![1, 2, 3, 4];
    let v2 = vec![5, 6, 7, 8];
    let mut v3 = vec![9, 10];
    println!("Stack address of v1 is: {:p}", &v1);
    println!("Stack address of v2 is: {:p}", &v2);
    v1.insert(1, 10);
    println!("{:?}", v1);
    v1.remove(0);
    println!("{:?}", v1);
    v1.append(&mut v3);
    println!("{:?}", v1);
    println!("{:?}", v3);

    // Box pointer
    let t1 = (10, String::from("hello"));
    // タプルのポインタ
    println!("Stack address of tuple data is: {:p}", &t1);
    // 10のポインタは同じ
    println!("t1.0のポインタ: {:p}", &t1.0);
    println!("t1.1のポインタ: {:p}", &t1.1);
    println!("Heap memory address of t1.1: data is: {:p}", t1.1.as_ptr());
    println!("len {}", t1.1.len());
    println!("cap {}", t1.1.capacity());
}
