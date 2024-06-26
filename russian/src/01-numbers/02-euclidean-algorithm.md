# 1.2 Алгоритм Евклида

## Положительные числа

<dl>
  <dt>Задача 1.2.1.</dt>
  <dd>
  На олимпиаду по программированию приехали 210 девочек и 252 мальчика.
  Мы хотим разбить их на команды так, чтобы во всех командах было одинаковое количество девочек и одинаковое количество мальчиков, например, везде 4 девочки и 7 мальчиков.
  Как много команд мы можем составить? Интересует максимально возможное число.
  </dd>
</dl>

Каково бы ни было количество команд, на него должны делиться и число 210, и число 252.
Нам нужен общий делитель двух чисел, а, учитывая, что нас интересует максимальное возможное число, речь идёт а *Наибольшем Общем Делителе* (НОД или, по-английски, GCD).

$$
gcd(210, 252) = 42
$$

Решение — мы можем составить 42 команды, в каждой из которых будет 5 девочек и 6 мальчиков.

Вы знаете ответ, потому что я посчитал за вас, чему равен $gcd(210, 252)$.

А как вычислить НОД самостоятельно?

Существует сложный алгоритм вычисления.
Он заключается в том, что мы разбиваем оба числа на простые множетели.
$210 = 2 \times 3 \times 5 \times 7$ и $252 = 2^2 \times 3^2 \times 7$.

Для вычисления НОД перемножьте числа, которые встречаются в обоих разложениях с наименьшей степенью.
$2 \times 3 \times 7 = 42$.

Разбиение на простые множества кажется (возможно, пока) нетривиальной задачей.

К счастью для нас, в глубокой древности люди научились вычислять НОД простым способом, а известный математик Евклид его записал.
Теперь этот способ называют *алгоритмом Евклида*.

Для начала выясним несколько фактор относительно наибольшего общего делителя. **Факт первый:** не имеет значения, в каком порядке рассматривать числа, для которых мы вычисляем НОД.

$$
\gcd(a, b) = \gcd(b, a)
$$

Для упрощения рассуждений будем считать, что в записи $\gcd(a, b)$ параметр $a$ больше или равен $b$.
Если это не так, мы всегда можем поменять параметры местами.
Далее **факт второй**.

$$
\gcd(a, b) = \gcd(b, a \mod b)
$$

Здесь $a \mod b$ — остаток от деления $a$ на $b$.
Почему это так?

Представим $a$ в виде $a = b q + r$, здесь $q$ — частное от деления $a$ на $b$, а $r$ — остаток.
У $a$ и $b$ есть общие делители, по меньше мере, оба числа всегда делятся на $1$.
Если на общий делитель делится $b$, значит, будет делиться и $b q$.
Тогда, с неизбежностью, на этот делитель должен делиться и остаток $r$.

Это рассуждение касается всех общих делителей $a$ и $b$, значит, $r$ делится на любой общий делитель, включая наибольший. Следовательно, $\gcd(a, b) = \gcd(b, r)$, где $r = a \mod b$.

Обратим внимание, что пара чисел $b$ и $r$ «меньше», чем $a$ и $b$. По найшей договорённости, $a >= b$, а $r$, который является остатком от деления на $b$ всегда меньше $b$ ($0 <= r < b$).

Применяя трюк в вычислением остатка и получая меньшие пары чисел, мы, в конце концов, доберёмся до вырожденного случая, к которому относится **третий факт**.

$$
gcd(a, 0) = a
$$

Возможно, он выглядит странно, но рассуждения нас не подведут: и число $a$, и число $0$ можно разделить на $a$.

Алгоритм Евклида заключается в том, что мы вычисляем остатки от деления, пока не достигнем вырожденного случая.
Мы знаем, что на каждом шаге НОД будет оставаться неизменным (второЙ факт).
На последнем (вырожденном) шаге, второй параметр всегда будет равен $0$, а первый — искомому НОД.

```rust
pub fn gdc(a: u32, b: u32) -> u32 {
    let (mut a, mut b) =
        if a > b {
            (a, b)
        } else {
            (b, a)
        };

    while b != 0 {
        (a, b) = (b, a % b);
    }

    a
}
```

В первых строках функции мы гарантируем, что `a >= b`, а, если это не так, меняем параметры местами.
Здесь и в основном цикле для обмена значениями мы пользуемся развитой системой типов Rust, а именно кортежами.

Запись `(a, b) = (b, a % b)` означает, что одновременно будут выполнены присвоения `a = b` и `b = a % b`, при этом Rust проследит за тем, чтобы они друг на друга не влияли.

В языках без кортежей нам бы пришлось вводить вспомогательную переменную, которую традиционно называют `t` или `tmp` от англиского слово *temporary* — временный.

```rust
    while b != 0 {
        let tmp = b;
        b = a % b; // мы потеряли старое значение b
        a = tmp; // поэтому здесь присваиваем переменную tmp, где сохранили старое значение b
    }
```

Алгоритм Евклида — хорошая иллюстрация для понятия алгоритма в целом.
С одной стороны, он не примитивный, содержит условие и цикл, требует обоснования корректности, с другой — он всё ещё остаётся простым.

## Упражнения

<dl>
  <dt>Задача 1.2.2.</dt>
  <dd>
  Мы рассмотрели алгоритм Евклида для положительных чисел.
  Однако, его можно применять и к любым целым числам.
  По определению, $\gcd(a, b) = \gcd(|a|, |b|)$.
  Реализуйте алгоритм Евклида для чисел типа `i32`.
  </dd>
</dl>
