# 2. Символы ASCII

В XXI неприлично использовать символы ASCII — все давно перешли на Unicode.
Впрочем, они хорошо подходят, чтобы изучать некоторые аспекты программирования.

ASCII (читается *аски*) — *American Standard Code for Information Interchange*.

Этот стандарт закрепляет численные коды за основными печатными символами (включая буквы английского алфавита) и некоторыми широко используемыми управляющими символами.

Стандарт разрабатывался несколько лет и в современном виде опубликован в 1967 году.
В то время широкое распространение имели *телетайпы* — электронные пишущие машинки, используемые для ввода и вывода информации вместо *терминалов*.

Управляющие символы позволяли двигать каретку телетайпа, возвращать её в начало строки переводить на следующую строку.

До сих пор мы называем их *возвратом каретки* (*carriage return*) и *переводом строки* (*line feed*), хотя в наше время не только телетайпы но и классические алфавитно-цифровые терминалы давно стали исторей.

Классическая таблица символов ASCII описывает 128 значений, то есть 7 битов из байта. Старший бит предполагалось использовать для контроля чётности, но в последствии от этой идеи отказались. ASCII стали расширять, в разных странах по разному, чтобы кодировать символы псевдографики и буквы местного алфавита.

Оригинальная таблица ASCII выглядит так.

| | | | | | | | | | | | | | | | | | | | | | | | | | | | | | | | |
|----:|:---:|----:|:---:|----:|:---:|----:|:---:|----:|:---:|----:|:---:|----:|:---:|----:|:---:|----:|:---:|----:|:---:|----:|:---:|----:|:---:|----:|:---:|----:|:---:|----:|:---:|----:|:---:|
|  0|NUL|  1|SOH|  2|STX|  3|ETX|  4|EOT|  5|ENQ|  6|ACK|  7|BEL|  8| BS|  9| HT| 10| LF| 11| VT| 12| FF| 13| CR| 14| SO| 15| SI|
| 16|DLE| 17|DC1| 18|DC2| 19|DC3| 20|DC4| 21|NAK| 22|SYN| 23|ETB| 24|CAN| 25| EM| 26|SUB| 27|ESC| 28| FS| 29| GS| 30| RS| 31| US|
| 32|   | 33| ! | 34| " | 35| # | 36| $ | 37| % | 38| & | 39| ' | 40| ( | 41| ) | 42| * | 43| + | 44| , | 45| - | 46| . | 47| / |
| 48| 0 | 49| 1 | 50| 2 | 51| 3 | 52| 4 | 53| 5 | 54| 6 | 55| 7 | 56| 8 | 57| 9 | 58| : | 59| ; | 60| < | 61| = | 62| > | 63| ? |
| 64| @ | 65| A | 66| B | 67| C | 68| D | 69| E | 70| F | 71| G | 72| H | 73| I | 74| J | 75| K | 76| L | 77| M | 78| N | 79| O |
| 80| P | 81| Q | 82| R | 83| S | 84| T | 85| U | 86| V | 87| W | 88| X | 89| Y | 90| Z | 91| [ | 92| \ | 93| ] | 94| ^ | 95| _ |
| 96| ` | 97| a | 98| b | 99| c |100| d |101| e |102| f |103| g |104| h |105| i |106| j |107| k |108| l |109| m |110| n |111| o |
|112| p |113| q |114| r |115| s |116| t |117| u |118| v |119| w |120| x |121| y |122| z |123| { |124| \||125| } |126| ~ |127|DEL|

Первые 32 символа, также, как и последний (с кодом 127) — управляющие. Пустая ячейка рядом с кодом 32 означает, что это *пробел*.
Печать пробела, сдвигала каретку вправо на одну позицию, не производя никакого видимого эффекта. Для сдвига каретки влево использовался управляющий символ с кодом 8 — backspace.

Сейчас нажатие клавиши Backspace стирает символ перед курсором, но на телетайпах он, конечно, оставался на бумаге.
Программисты тех лет изобретали трюки, позволявшие печатать, скажем, зачёркнутый текст.
Чтобы зачеркнуть слово PROGRAM, надо было напечатать буквы P, R, O, G, R, A, M, затем 7 символов BS (устоявшийся акроним для символа с кодом 8) и ещё 7 символов дефис `-`, которые печатались поверх букв.

Получалось ~PROGRAM~.

Что ж, исследуем символы ASCII и узнаем, что можно с ними делать.