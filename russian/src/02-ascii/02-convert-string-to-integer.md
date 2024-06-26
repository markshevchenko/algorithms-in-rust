# 2.2. Переводим строку в число

Процессор легко оперирует числами — двубайтными, четырёхбайтными, восьмибайтными.
Однако в тексте программ числа записаны символами и перед вычислениями их надо перевести в доступную для компьютера форму.

Строку `"12345"` в число `12345`.

Этот процесс называют иногда *конвертацией*, иногда — *лексическим анализом* (parsing).
Также говорят, что это *приведение* типа.

В Rust это *анализ*, поскольку соответсвующий метод называются `parse<T>()`.
Он работает с Unicode-строками, а мы напишем версию для ASCII-строк.

В Rust ASCII-строка — это на самом деле массив байтов. Мы можем в явном виде перечислить коды символов, либо можем записать строку с префиксом **b**. `b"Bar"` — ASCII-строка, содержащая символы `b'B'`, `b'a'` и `b'r'`.

## Идея алгоритма

Предоложим, мы конвертируем в число строку `"153"` и к настоящему моменту уже сконвертировали цифры $1$ и $5$. Переменная, в которой мы *собираем* значение — *аккумулятор* — хранит число $15$.

Если далее снова следует цифра, значит, речь идёт уже не о числе $15$, а о числе $150$ плюс эта самая цифра.
Мы должны умножить аккумулятор на $10$ (как бы сдвинуть цифры на одну позицию влево) и прибавить найденную цирфу (как бы вписать её в пустую позицию справа).

Однако, код цифры `'3'`, в ASCII равен не 3, а 51. Чтобы получить 3, надо из `b'3'` вычесть код символа `b'0'`, то есть 48.

```rust
    // ...
    let mut accumulator: u32 = 0;
    let mut i = 0;

    while i < s.len() && is_digit(s[i]) {
        let next_digit = u32::from(s[i] - b'0');
        accumulator = 10 * accumulator + next_digit;

        i += 1;
    }
    // ...
```

Здесь мы предполагаем, что исходная строка с цифрами находится в переменной `s`. У нас есть индексная переменная `i`, с помощью которой мы можем обратиться к каждому символу, начиная с самого первого (имеет индекс `0`), до самого последнего (индекс `s.len() - 1`).

Из-за явного преобразования типов в Rust, мы не можем непосредственно прибавить значение типа `u8` к значению типа `u32`, поэтому нам требуется метод `u32::from()`.

В самом начале аккумулятор равен $0$.
Цикл устроен так, что когда будет прочитана первая цифра, например, `1`, $0$ будет умножен на $10$ и это всегда даст результат $0$.

Мы можем обрабатывать первую цифру также, как и все остальные цифры, не реализуя *особый случай*.

Какой результат должна возвращать функция анализа?
Если в строке совсем нет цифр, то и результата преобразования тоже нет.
Подходящим типом для выражения этой идеи является `Option<u32>`.
Таким образом, функция целиком приобретает такой вид.

```rust
pub fn parse_u32(s: &[u8]) -> Option<u32> {
    if s.len() == 0 || !is_digit(s[0]) {
        return None;
    }

    let mut accumulator: u32 = 0;
    let mut i = 0;

    while i < s.len() && is_digit(s[i]) {
        let next_digit = u32::from(s[i] - b'0');
        accumulator = 10 * accumulator + next_digit;

        i += 1;
    }

    Some(accumulator)
}
```

На вход функция получает массив байт, по сути — строку символов ASCII.
Если массив пуст, или начинается не с цифры, функция возвращает `None` — пустой результат.
Если массив не пуст и в нём есть хотя бы одна цифра, функция *перебирает* все цифры, одновременно пересчитывая значение в аккумуляторе.
Когда цикл завершается, в аккумуляторе находится искомый результат.
Мы возвращаем `Some(accumulator)`, потому что результат нашей функции — *опциональный* тип, которой должен либо не содержать ничего (`None`), либо содержать что-то (`Some`).

## Переполнение

Функция работает корректно, пока мы не попытаемся сконвертировать достаточно большое число, например, $5\,000\,000\,000$. Тогда программа остановися с сообщением об ошибке.

```text
thread 'main' panicked at src\ascii\mod.rs:45:23:
attempt to multiply with overflow
```

Тип `u32` хранит 32-хбитные числа без знака.
Максимальное возможное значение `u32::MAX` равно $4\,294\,967\,295$ или $2^32-1$.
Если число в строке превышает `u32::MAX`, на очередном шаге заполнения аккумулятора происходит *переполнение* (*overflow*).

Чтобы справиться с этой ошибкой, мы должны «читать» из входной строки цифры, пока они помещаются в `u32::MAX`.

Если при добавлении цифры происходит переполнение, мы завершаем разбор строки и возвращаем число, которое успели прочитать к настоящему моменту.

У нас для *накопления* используется составная формула.

```rust
 `accumulator = 10 * accumulator + next_digit`.
```

Проверку придётся делать в два этапа. Давайте вначале убедиться, что прибавление цифры не приведёт к переполнению.

```rust
if 10 * accumulator + next_digit > u32::MAX {
    return Some(accumulator);
}
```

К сожалению, код в таком виде не работает, потому что условие всегда будет *ложным*.
Числа, хранящиеся в `u32` не могут быть больше `u32::MAX`.
Выражение в левой части всё равно может привести к переполнению, а, значит, и к остановке программы.
Чтобы избавиться от переполнения, перенесём слагаемое `next_digit` в правую часть.

```rust
if 10 * accumulator > u32::MAX - next_digit {
    return Some(accumulator);
}
```

Точно также, мы должны убедиться, что `10 * accumulator` меньше, чем `u32::MAX`.
Как и в случае со сложением, нам надо перенести $10$ из левой части в правую.

```rust
if accumulator > u32::MAX / 10 || 10 * accumulator > u32::MAX - next_digit {
    return Some(accumulator);
}
```

Полный код безопасного преобразования выглядит так.

```rust
pub fn parse_u32(s: &[u8]) -> Option<u32> {
    if s.len() == 0 || !is_digit(s[0]) {
        return None;
    }

    let mut accumulator: u32 = 0;
    let mut i = 0;

    while i < s.len() && is_digit(s[i]) {
        let next_digit = u32::from(s[i] - b'0');
        if accumulator > u32::MAX / 10 || 10 * accumulator > u32::MAX - next_digit {
            return Some(accumulator);
        }

        accumulator = 10 * accumulator + next_digit;
        
        i += 1;
    }

    Some(accumulator)
}
```

В реальном программировании часто приходится проверять граничные условия, что усложняет программу, но является необходимой частью кода.

Функция `parse_u32()` при конвертации строки `"4294967296"`, читает не 10 символов, а 9 и возвращает `Some(429496729)`.
$4\,294\,967\,296$ на единицу больше `u32::MAX` и приводит к переполнению.
