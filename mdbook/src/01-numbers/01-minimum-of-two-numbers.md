# 1.1. Минимум двух чисел

## Целые числа

<dl>
  <dt>Задача 1.1.1.</dt>
  <dd>
  Виктор ходит на обед либо в кафе DevOps, либо в ресторан «Ни рыба, ни мясо».
  В кафе у него накопилась скидка в 20%, бизнес-ланч там стоит 250 рублей.
  Скидка в ресторане составляет 30%, но бизнес-ланч стоит 400 рублей.
  У Виктора осталось не так много денег до зарплаты и он не хочет тратить лишние деньги.
  Помогите ему определить самый дешёвый бизнес-ланс.
  </dd>
</dl>

Эта задача может показаться совершенно примитивной.
Трудно представить, что здесь может идти речь о каких бы то ни было алгоритмах.
Между тем, определение процентов и минимума двух чисел — хорошие примеры простых алгоритмов.

Сначала разберёмся с процентами.
Чтобы посчитать $p$ процентов от числа $n$, нужно $n$ умножить на $p/100$.

```rust
pub fn percent(n: i32, p: i32) -> i32 {
    n * p / 100
}
```

Простейшие алгоритмы представляют из себя последовательные вычисления без условий и циклов.

Теперь мы без труда можем посчитать процент от стоимости бизнес-ланча.

```rust
let result = percent(250, 20); // 50
```

Впрочем, нас интересует не сумма скидки, а стоимость бизнес-ланча с учётом скидки.
Вместо $20%$ мы должны использовать дополнение до $100$, то есть $(100 - 20)%$.

```rust
let result = percent(250, 100 - 20); // 200
```

Разобравшись с процентами, напишем функцию определения минимума.

```rust
pub fn min(a: i32, b: i32) -> i32 {
    if a < b {
        a
    } else {
        b
    }
}
```

Я знаю, как ужасно звучит, когда автор пишет сложный код и утверждает, что он очевиден, но здесь код и правда очевиден.
Из двух переданных параметров функция `min()` возвращает тот, который меньше, а если они равны, то второй.

```rust
let result = min(percent(250, 100 - 20), percent(400, 100 - 30)); // 200
```

## Обобщение

Rust — язык с сильной типизацией.
Если мы написали функцию, которая работает с целыми знаковыми числами `i32`, мы не можем вызывать её для беззнаковых чисел `u32` или больших чисел `i64`.

В идеале нам бы хотелось написать универсальную функцию, которая будет работать со всеми типами, которые можно сравнить друг с другом.
В Rust это совсем нетрудно сделать.

```rust
pub fn min2<T>(a: T, b: T) -> T where T: Ord {
    if a < b {
        a
    } else {
        b
    }
}
```

Это обобщённый (generic) код похож на обобщённый код из C++, Java и C#, так что, кажется, не требует объяснений.
Если вы никогда раньше не сталкивались с обобщённым типами, прочитайте [нужную главу](https://doc.rust-lang.ru/book/ch10-01-syntax.html) из учебника по языку Rust.

В данном коде нас дожен заинтересовать *типаж* `Ord`.
Его реализуют все типы, которые можно сравнивать, в том числе и целые числа.
Если тип `T` реализует `Ord` (на это указывает конструкция `where T: Ord`), мы можем применить оператор "меньше" (<) к переменным типа `T`.

Эта магия работает благодаря компилятору Rust, для которого `Ord` — особый типаж. Без магии код функции `min2<T>()` выглядит так.

```rust
pub fn min2<T>(a: T, b: T) -> T where T: Ord {
    use std::cmp::Ordering;

    match a.cmp(&b) {
        Ordering::Less => a,
        Ordering::Equal => b,
        Ordering::Greater => b,
    }
}
```

Новая функция `min2<T>()` делает то же самое, что и старая функция `min`, но, в отличие от первой, может работать с любыми сравнимыми объектами, даже с теми, которые мы напишем сами.

## Сведние задачи к уже решённой

Предположим, в истории о бизнес-ланче Виктора надо сравнить не два, а три заведения со скидками.
Тогда нам потребуется функция, которая возвращает минимум из трёх чисел.

Её нетрудно написать с помощью оператора `if/else`, но этот код будет громоздким.
Программисты, решая чуть более сложную задачу, часто используют существующее решение, так сказать, сводят новую задачу к решённой.
Вместо того, чтобы писать вложенные операторы `if/else`, вызовем функцию `min2<T>` два раза.

```rust
pub fn min3<T>(a: T, b: T, c: T) -> T where T: Ord {
    min2(a, min2(b, c))
}
```

Такое решение не только занимает меньше места, но и проще читается.
В этой функции мы сначла находим наименьшее из чисел `b` и `c`, а затем победителя сравниваем с `a`.

## Сравнение чисел с плавающей запятой

Если вам доводилось работать с числами с плавающей запятой, вы знаете, что они немного *магические*.
Они живут по каким-то своим законам, с которыми не так то просто разобраться.

Если мы попробуем сравнить два числа с плавающей запятой, наша программа просто не скомпилируется.

```rust
let result = min2(2.7182, 3.1415); // the trait `Ord` is not implemented for `{float}`
```

Сообщение об ошибке звучит странно: типаж `Ord` не реализован для типа `float`. Почему Rust не позволяет сравнивать числа с плавающей запятой?

В этом есть смысл. Типаж `Ord` доступен только для тех типов, в которых порядок определён для любых значений. Сюда попадают все целые типы, символы ASCII, даты и строки.

Однако у чисел с плавающей запятой есть несколько специальных значений, одно из которых это *Not a Number* или *NaN*. Оно возникает в процессе вычислений, если мы выполняем некорректную операцию, например, деление нуля на ноль.

Поскольку *NaN* не является числом, его нельзя сравнивать с другими числами с плавающей запятой.
Именно поэтому типаж `Ord` не реализован для типа `float`.
Что же делать?

Для представления типов, которые можно сравнивать, но не всегда, в Rust существует типаж `PartialOrd`.

```rust
pub fn partial_min2<T>(a: T, b: T) -> Option<T> where T: PartialOrd {
    use std::cmp::Ordering;

    match a.partial_cmp(&b) {
        Some(Ordering::Less) => Some(a),
        Some(Ordering::Equal) => Some(b),
        Some(Ordering::Greater) => Some(b),
        None => None,
    }
}
```

Обратите внимание на тип `Option<T>`.
Этот обобщённый тип представляет *опциональное* значение.
В контексте нашей функции это значит, что если параметры `a` и `b` сравнимы, она возвращает наименьший, а если нет — не возвращает ничего.

В таких языках, как Pascal, C, C++, Java и C# для представления отсутствующих величин часто используют указатель `null` (в Pascal он называется `nil`, в C — `NULL`).
К сожалению, с ним постоянно возникают проблема *NPE*, по названию исключения `NullPointerException` из Java.

В Rust такой проблемы возникнуть не может, поскольку нельзя "вытащить" значение из `Option<T>` не проверив предварительно, что оно там есть[^1].

## Упражнения

<dl>
  <dt>Задача 1.1.2.</dt>
  <dd>
  Напишите обобщённую версию функции `max2<T>()`, которая возвращает наибольший из двух параметров.
  </dd>
  <dt>Задача 1.1.3.</dt>
  <dd>
  Напишите обобщённую версию функции `partial_min3<T>()` которая возвращает наименьший из трёх параметров или `None`, если среди параметров есть такие, которые нельзя сравнивать.
  </dd>
</dl>

[^1]: Это не совсем так.
Вы вполне можете устроить панику, "вытащив" значение без всякой проверки с помощью метода `unwrap()`.
Метод удобен при прототирировании или изучении языка, но не вызывайте его в продуктовом коде.