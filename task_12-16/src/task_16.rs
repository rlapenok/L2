//функция, которая производит отпрааку значений в канал и возвращает часть канала, отвечающую на прием
fn as_chan(vs: &[i32]) -> std::sync::mpsc::Receiver<i32> {
    //создание канала
    let (tx, rx) = std::sync::mpsc::channel();
    //содание потока
    let handle = std::thread::spawn({
        //преобраование среза в Vec
        let vs = vs.to_owned();
        //move Vec
        move || {
            //иттерация по значениям
            for v in vs {
                //отправка в канал
                tx.send(v).unwrap();
                //поток засыпает на 1 секунду
                std::thread::sleep(std::time::Duration::from_secs(1))
            }
            //drop отправителя
            drop(tx);
        }
    });
    //ожидание завершения потока
    handle.join().unwrap();
    //возврат приемника
    rx
}

//функция принимает два приемника, читает из каждого канала, и записывает результы прочтения из этих двух 
//каналов в другой канал
fn merge(
    a: std::sync::mpsc::Receiver<i32>,
    b: std::sync::mpsc::Receiver<i32>,
) -> std::sync::mpsc::Receiver<i32> {
    //создание канала с результатми
    let (tx, rx) = std::sync::mpsc::channel();
    //создание флага о завершение канала А
    let mut a_done = false;
    //создание флага о завершение канала В
    let mut b_done = false;
   //бесконечный цикл, цитающий поочередно из каналов 
    loop {
        // match потытки принять значения
        match a.try_recv() {
            //отправка в канал с результатами
            Ok(i) => {
                tx.send(i).unwrap();
            }
            // выставление флага для канала А, что он закрыт
            Err(_) => {
                a_done = true;
            }
        }
        //аналогично описанию выше
        match b.try_recv() {
            Ok(i) => {
                tx.send(i).unwrap();
            }

            Err(_) => {
                b_done = true;
            }
        }
        //условие, при котором звершается бесконечный цикл
        if a_done && b_done {
            break;
        }
    }

    rx
}

fn main() {
    let a = as_chan(&vec![1, 3, 5, 7]);

    let b = as_chan(&vec![2, 4, 6, 8]);

    let c = merge(a, b);

    for v in c.iter() {
        println!("{v:?}");
    }
}
