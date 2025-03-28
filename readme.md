# Документация для `envsub-rs`

`envsub-rs` — это утилита для подстановки переменных окружения в текстовые файлы.

## Установка
*macos*
```bash
curl -LO https://github.com/ibatanov/envsub-rs/releases/download/release-0.1.0/envsub-rs-macos.zip && unzip envsub-rs-macos.zip && sudo mv envsub-rs /usr/local/bin/ && chmod +x /usr/local/bin/envsub-rs
```
*ubuntu*
```bash
wget https://github.com/ibatanov/envsub-rs/releases/download/release-0.1.0/envsub-rs-ubuntu.zip && unzip envsub-rs-ubuntu.zip && sudo mv envsub-rs /usr/local/bin/ && chmod +x /usr/local/bin/envsub-rs
```

После выполнения команды проверьте установку:
```bash
envsub-rs --version
envsub-rs --help
```

Удалите архив `envsub-rs-*.zip`, если он остался в текущей директории:
```bash
rm envsub-rs-*.zip
```

## Использование
```
echo 'Test and ${USER2:-admin} and ${USER:-guest}' | cargo run -- --no-unset --no-empty
Test and admin and guest
```
### Основной синтаксис

```bash
envsub-rs [OPTIONS]
```

### Опции

- `-i, --input <FILE>`: Указывает входной файл. Если не указан, данные читаются из стандартного ввода (stdin).
- `-o, --output <FILE>`: Указывает выходной файл. Если не указан, результат выводится в стандартный вывод (stdout).
- `--no-unset`: Если установлено, утилита завершится с ошибкой, если переменная окружения не установлена.
- `--no-empty`: Если установлено, утилита завершится с ошибкой, если переменная окружения установлена, но пуста.

### Примеры

#### Подстановка переменных из файла и вывод результата в stdout

```bash
envsub-rs -i input.txt
```

#### Подстановка переменных из файла и запись результата в другой файл

```bash
envsub-rs -i input.txt -o output.txt
```

#### Подстановка переменных из stdin и вывод результата в stdout

```bash
cat input.txt | envsub-rs
```

#### Подстановка переменных с проверкой на пустые и не установленные переменные

```bash
envsub-rs -i input.txt --no-unset --no-empty
```

### Пример использования в скрипте

```bash
#!/bin/bash

export NAME="World"
export GREETING="Hello"

envsub-rs -i template.txt -o output.txt
```

Где `template.txt` содержит:

```
${GREETING}, ${NAME}!
```

После выполнения скрипта, `output.txt` будет содержать:

```
Hello, World!
```

### Поддерживаемые шаблоны

- `${VAR}`: Подставляет значение переменной `VAR`. Если переменная не установлена, подставляет пустую строку.
- `${VAR:-default}`: Подставляет значение переменной `VAR`. Если переменная не установлена, подставляет значение `default`.
- `$VAR`: Подставляет значение переменной `VAR`. Если переменная не установлена, подставляет пустую строку.

### Ошибки

Если утилита завершается с ошибкой, она возвращает соответствующий код ошибки и сообщение:

- `Variable not set`: Возникает, если переменная не установлена и установлена опция `--no-unset`.
- `Variable is empty`: Возникает, если переменная установлена, но пуста, и установлена опция `--no-empty`.
- `Parsing error`: Возникает при ошибках чтения/записи файлов или других ошибках парсинга.

### Заключение

`envsub-rs` предоставляет простой и гибкий способ подстановки переменных окружения в текстовые файлы. Используйте его для автоматизации задач, связанных с конфигурацией и шаблонизацией.
