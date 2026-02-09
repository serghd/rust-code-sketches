pub fn evaluate_iterators_1() {
    let vec = vec![1, 2, 3];
    let vec_a = vec.iter().map(|x| x + 1).collect::<Vec<i32>>();
    let vec_b = vec.into_iter().map(|x| x + 10).collect::<Vec<i32>>();
    let mut vec_c = vec![1, 2, 3];
    vec_c.iter_mut().for_each(|x| *x *= 10);

    println!("{:?}", vec_a);
    println!("{:?}", vec_b);
    println!("{:?}", vec_c);
}