fn main() {
    //создаение канала небуфф передающего i32
    let (tx, rv) = std::sync::mpsc::channel::<i32>();
    //сощдание потока, который в цикле от 0 до 9 вкл.последовательно пишет в канал
    let handle = std::thread::spawn(move || {
        for i in 0..10 {
            tx.send(i).unwrap();
        }
    });
    //ожидание завершения потока
    handle.join().unwrap();
    //чтение из канала и последовательный вывод в консоль
    for i in rv.iter() {
        println!("{i:?}");
    }
}
