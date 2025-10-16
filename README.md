# Extractor Cookies

![Rust](https://img.shields.io/badge/Rust-1.80.0-orange?logo=rust)
![License](https://img.shields.io/badge/License-MIT-blue)

## English Version

### Description

Extractor Cookies is a Rust application for extracting and sorting cookies from browser logs. It allows searching for cookies by keywords (e.g., "steam" or "youtube"), saving them into separate folders or files, with name sanitization to avoid file system errors. The app features a simple graphical interface (based on egui) and supports saving modes: only cookies, folders with cookies, or full logs.

The project is designed for log analysis, extracting relevant data, and organizing it. It handles files like Cookies.txt, History.txt, and others, automatically detecting the browser and filtering cookies by services.

### Installation

1. Ensure you have Rust installed (version 1.80+ recommended).
2. Clone the repository:
   ```
   git clone https://github.com/yourusername/extractor-cookies.git
   cd extractor-cookies
   ```
3. Build and run:
   ```
   cargo build
   cargo run
   ```

Dependencies (from Cargo.toml):
- egui for UI
- Other standard Rust libraries (std::fs, std::path, etc.)

### Usage

1. Launch the app with `cargo run`.
2. In the interface, enter search keywords (e.g., "steam, youtube").
3. Specify the logs directory path (default: current logs folder).
4. Choose a saving mode:
   - **OnlyCookies**: Saves cookies to TXT files.
   - **FolderPlusCookies**: Creates folders for each service with cookies and logs.
   - **Folder**: Copies full logs to separate folders.
5. Click "Extract" — results will be saved in the `results` folder.

Example: Searching for YouTube and Steam cookies will create separate folders with relevant data, without mixing.

### Features

- **Cookie Search and Filtering**: By domains, names, and values.
- **Name Sanitization**: Automatic replacement of invalid characters for Windows.
- **Multi-Query Support**: Separate folders for each service.
- **Graphical Interface**: Simple and intuitive.
- **Error Handling**: Protection against errors like "InvalidFilename" or access issues.

### Contributing

If you want to contribute:
1. Fork the repository.
2. Create a branch: `git checkout -b feature/new-feature`.
3. Commit changes: `git commit -m 'Added new feature'`.
4. Push: `git push origin feature/new-feature`.
5. Create a Pull Request.

### License

This project is licensed under the MIT License. See the LICENSE file for details.

---

## Русская Версия

### Описание

Extractor Cookies — это приложение на Rust для извлечения и сортировки куки из логов браузеров. Оно позволяет искать куки по ключевым словам (например, "steam" или "youtube"), сохранять их в отдельные папки или файлы, с учетом санитизации имен для избежания ошибок файловой системы. Приложение имеет простой графический интерфейс (на базе egui) и поддерживает режимы сохранения: только куки, папки с куки или полные логи.

Проект разработан для анализа логов, извлечения релевантных данных и их организации. Он обрабатывает файлы вроде Cookies.txt, History.txt и других, автоматически определяя браузер и фильтруя куки по сервисам.

### Установка

1. Убедитесь, что у вас установлен Rust (рекомендуется версия 1.80+).
2. Клонируйте репозиторий:
   ```
   git clone https://github.com/yourusername/extractor-cookies.git
   cd extractor-cookies
   ```
3. Соберите и запустите:
   ```
   cargo build
   cargo run
   ```

Зависимости (из Cargo.toml):
- egui для UI
- Другие стандартные библиотеки Rust (std::fs, std::path и т.д.)

### Использование

1. Запустите приложение с помощью `cargo run`.
2. В интерфейсе введите ключевые слова для поиска (например, "steam, youtube").
3. Укажите путь к директории с логами (по умолчанию: текущая папка с логами).
4. Выберите режим сохранения:
   - **OnlyCookies**: Сохраняет куки в TXT-файлы.
   - **FolderPlusCookies**: Создает папки для каждого сервиса с куки и логами.
   - **Folder**: Копирует полные логи в отдельные папки.
5. Нажмите "Extract" — результаты сохранятся в папке `results`.

Пример: Поиск куки для YouTube и Steam создаст отдельные папки с релевантными данными, без смешивания.

### Функции

- **Поиск и фильтрация куки**: По доменам, именам и значениям.
- **Санитизация имен**: Автоматическая замена недопустимых символов для Windows.
- **Поддержка нескольких запросов**: Отдельные папки для каждого сервиса.
- **Графический интерфейс**: Простой и интуитивный.
- **Обработка ошибок**: Защита от ошибок вроде "InvalidFilename" или доступа.

### Вклад

Если хотите внести изменения:
1. Форкните репозиторий.
2. Создайте ветку: `git checkout -b feature/новая-функция`.
3. Зафиксируйте изменения: `git commit -m 'Добавлена новая функция'`.
4. Отправьте: `git push origin feature/новая-функция`.
5. Создайте Pull Request.

### Лицензия

Этот проект лицензирован под MIT License. Подробности в файле LICENSE.

---

If you have additional details (e.g., screenshots or specific features), let me know, and I'll update the README!