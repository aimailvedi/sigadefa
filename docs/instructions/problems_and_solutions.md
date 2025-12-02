# Инструкция: журнал проблем и решений (SIGADEFA)

Этот файл — место, где фиксируются **технические проблемы** и
их решения, найденные в диалоге «человек ↔ ИИ».

## Пример 1. Проблема с кавычками в Python (SyntaxError)

**Симптом:**
- \SyntaxError: unexpected character after line continuation character\
- В файле \requency_model.py\ или \udio_export.py\ видны строки с \\"...\\.

**Причина:**
- В код попали экранированные кавычки из here-string PowerShell.

**Решение:**
1. Перезаписать файл через \Set-Content\ с обычными кавычками:
   - \"\ вместо \\\\"\.
2. Ещё раз проверить файл через \Get-Content\.
3. Запустить скрипт:
   - \python src/sigadefa/audio_export.py\.

---

## Пример 2. Нет модуля numpy

**Симптом:**
- \ModuleNotFoundError: No module named 'numpy'\.

**Решение:**
1. Установить numpy:
   - \pip install numpy\.
2. Повторно запустить:
   - \python src/sigadefa/audio_export.py\.

---

## Пример 3. Проблемы с MkDocs и темой material

**Симптомы:**
- Ошибка: «Unrecognised theme name: 'material'».
- Или предупреждения, что файлы из \
av\ не найдены.

**Решение:**
1. Установить тему:
   - \pip install mkdocs-material\.
2. Убедиться, что пути в \mkdocs.yml\ указывают на файлы внутри \docs/\.

---

## Пример 4. Первый звук SIGADEFA не создаётся

**Симптомы:**
- Нет файла \sigadefa_first_sound.wav\.
- Ошибки в импортах или синтаксисе.

**Решение (общее):**
1. Проверить \udio_export.py\ и \requency_model.py\.
2. Убедиться, что:
   - импорт: \rom sigadefa.frequency_model import FrequencyModel\;
   - \FrequencyModel\ успешно создаёт частоты.
3. Запустить:
   - \python src/sigadefa/audio_export.py\.
4. Проверить наличие файла в корне проекта.

---

## Как использовать этот файл дальше

- После каждого серьёзного бага:
  - коротко описать симптом,
  - записать причину (когда понятна),
  - зафиксировать шаги решения.

Так мы создаём **память проекта** о типичных ошибках и не повторяем их.
