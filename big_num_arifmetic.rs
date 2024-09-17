fn main() {
    // Вводим числа в виде строк
    let a = "20000000000000000000000";
    let b = "30000000000000000000000";

    // Выполнение операций
    let sum = big_add(a, b);
    let difference = big_sub_with_sign(a, b);
    let product = big_mul(a, b);
    let division = big_div(a, b);

    // Вывод результатов
    println!("Число a: {}", a);
    println!("Число b: {}", b); 
    println!("Сумма: {}", sum);
    println!("Разность: {}", difference);
    println!("Произведение: {}", product);
    match division {
        Some(result) => println!("Частное: {}", result),
        None => println!("Деление на ноль невозможно"),
    }
}

// Функция для сложения больших чисел
fn big_add(a: &str, b: &str) -> String {
    let mut result = String::new(); // Строка для хранения результата
    let mut carry = 0; // Переменная для хранения переноса

    // Разворачиваем строки для удобства
    let mut a_chars = a.chars().rev();
    let mut b_chars = b.chars().rev();

    // Основной цикл выполняется, пока есть цифры в a или b или есть перенос
    while a_chars.clone().next().is_some() || b_chars.clone().next().is_some() || carry > 0 {
        // Используем next() для получения следующей цифры из a и b. Если цифр больше нет, возвращаем '0'.
        let digit_a = a_chars.next().unwrap_or('0').to_digit(10).unwrap();
        let digit_b = b_chars.next().unwrap_or('0').to_digit(10).unwrap();

        let sum = digit_a + digit_b + carry; // Суммируем текущие цифры и перенос
        result.push((sum % 10).to_string().chars().next().unwrap()); // Добавляем в результат остаток от деления суммы на 10
        carry = sum / 10; // В перенос записываем результат деления на 10 без остатка
    }

    // Разворачиваем и возвращаем результат
    result.chars().rev().collect()
}

// Функция для вычитания больших чисел с учетом знака
fn big_sub_with_sign(a: &str, b: &str) -> String {
    // Если b больше a
    if is_greater_or_equal(b, a) {
        return format!("-{}", big_sub(b, a)); // Возвращаем отрицательное значение
    }
    big_sub(a, b) // Возвращаем обычную разность
}

// Функция для вычитания больших чисел
fn big_sub(a: &str, b: &str) -> String {
    // Если b больше или равно a, возвращаем "0"
    if is_greater_or_equal(b, a) {
        return String::from("0");
    }

    let mut result = String::new(); // Строка для хранения результата
    let mut borrow = 0; // Переменная для хранения займа

    // Разворачиваем строки для удобства
    let mut a_chars = a.chars().rev();
    let mut b_chars = b.chars().rev();

    // Основной цикл выполняется, пока есть цифры в a или b или есть заем
    while a_chars.clone().next().is_some() || b_chars.clone().next().is_some() || borrow > 0 {
        let digit_a = a_chars.next().unwrap_or('0').to_digit(10).unwrap(); // Получаем следующую цифру a
        let digit_b = b_chars.next().unwrap_or('0').to_digit(10).unwrap(); // Получаем следующую цифру b

        // Вычитаем цифры, учитывая заем
        let mut current = digit_a as i32 - digit_b as i32 - borrow;

        if current < 0 { // Если текущий результат отрицательный
            current += 10; // Добавляем 10 для выполнения вычитания
            borrow = 1; // Устанавливаем заем
        } else {
            borrow = 0; // Обнуляем заем
        }

         // Преобразуем разность в строку, берем первый символ и добавляем его в результат
        result.push((current as u32).to_string().chars().next().unwrap());
    }

    // Удаляем нули и разворачиваем результат
    result.chars().rev().skip_while(|&digit| digit == '0').collect::<String>()
}

// Функция для умножения больших чисел
fn big_mul(a: &str, b: &str) -> String {
    let mut result = vec![0; a.len() + b.len()]; // Результат умножения

    // Итерация по цифрам числа a
    for (i, digit_a) in a.bytes().rev().enumerate() {
        let mut carry = 0; // Переменная для хранения переноса
        let digit_a = (digit_a - b'0') as usize; // Преобразуем символ в число

        // Итерация по цифрам числа b
        for (j, digit_b) in b.bytes().rev().enumerate() {
            let digit_b = (digit_b - b'0') as usize; // Преобразуем символ в число
            let product = digit_a * digit_b + carry + result[i + j]; // Перемножаем числа и добавляем перенос

            result[i + j] = product % 10; // Сохраняем в результат остаток от деления выражения на 10
            carry = product / 10; // В перенос записываем результат деления на 10 без остатка
        }
        result[i + b.len()] += carry; // Добавляем оставшийся перенос
    }

    // Разворачиваем вектор, пропускаем 0, преобразуем числа в символы, собираем символы в строку и возвращаем результат
    result.into_iter().rev().skip_while(|&digit| digit == 0).map(|d| (d as u8 + b'0') as char).collect::<String>()
}

// Функция для деления больших чисел
fn big_div(a: &str, b: &str) -> Option<String> {
    if b == "0" {
        return None; // Деление на ноль невозможно
    }

    if is_greater_or_equal(b, a) {
        return Some(String::from("0"));
    }

    let dividend = String::from(a); // Делимое
    let divisor = String::from(b); // Делитель
    let mut result = String::new(); // Результат деления
    let mut current = String::new(); // Текущий остаток

    for digit in dividend.chars() {
        current.push(digit); // Добавляем следующую цифру к текущему остатку

        // Находим, сколько раз делитель помещается в текущий остаток
        let mut count = 0;
        while is_greater_or_equal(&current, &divisor) {
            current = big_sub(&current, &divisor); // Вычитаем делитель из остатка
            count += 1; // Увеличиваем счетчик
        }

        result.push_str(&count.to_string()); // Добавляем к результату
    }

    // Удаляем нули
    result = result.trim_start_matches('0').to_string();
    
    // Если результат пустой, значит, результат равен 0
    if result.is_empty() {
        result = String::from("0");
    }

    Some(result) // Возвращаем результат
}

// Вспомогательная функция для проверки, больше ли одно число или равно другому
fn is_greater_or_equal(a: &str, b: &str) -> bool {
    if a.len() > b.len() {
        return true;
    } else if a.len() < b.len() {
        return false;
    }
    a >= b
}
