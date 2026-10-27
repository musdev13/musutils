# Contributing to musutils / Вклад в musutils

## EN

Thanks for your interest in the project!

### Structure

The library uses **independent module versioning**. Every module lives in its own directory with two generations side by side:

```
src/
├── lib.rs              # feature gating + available_modules() + inter-module dependency checks
├── types_macros.rs     # log! macro (always available, #[macro_export])
├── color/  { mod.rs, v1/, v2/ }
├── fs/     { mod.rs, v1/, v2/ }
├── http/   { mod.rs, v1/, v2/ }
├── os/     { mod.rs, v1/, v2/ }
├── types/  { mod.rs, v1/, v2/ }
└── vec/    { mod.rs, v1/, v2/ }
tests/                  # integration tests, one file per module (and per generation)
├── common/             # helpers (TempDir, HTTP test server, env lock)
├── color.rs
├── fs_v1_basic.rs
├── fs_v1_config.rs
├── fs_v2_basic.rs
├── fs_v2_config.rs
├── http_v1.rs
├── http_v2.rs
├── macros.rs
├── os.rs
├── types.rs
├── vec.rs
└── versions.rs         # integration tests for available_modules()
docs/                   # MkDocs site sources
├── index.md            # site landing page
├── README.md           # GitHub-facing overview
└── pages/
    ├── getting-started/
    ├── lib.md
    └── modules/<module>/<generation>.md
mkdocs.yml              # MkDocs configuration and navigation
Makefile                # make test / make lint / make check
```

`src/<mod>/mod.rs` is only for declarations and re-exports:

```rust
#[cfg(all(feature = "fs-v1", feature = "fs-v2"))]
compile_error!("features `fs-v1` and `fs-v2` are mutually exclusive; enable exactly one");

#[cfg(feature = "fs-v1")]
pub mod v1;
#[cfg(feature = "fs-v2")]
pub mod v2;

#[cfg(feature = "fs-v1")]
pub use self::v1::*;
#[cfg(all(feature = "fs-v2", not(feature = "fs-v1")))]
pub use self::v2::*;
```

Feature names follow `<module>-<generation>`: `fs-v1`, `fs-v2`, `vec-v1`, `vec-v2`, `http-v1`, `http-v2`, `color-v1`, `color-v2`, `os-v1`, `os-v2`, `types-v1`, `types-v2`. `default = []` — no module is enabled by default.

### Rules

1. **All changes go to the `dev` branch.** Open pull requests against `dev`, not `main`.

2. **Keep the modular structure.** New functionality goes into the appropriate module (`color/`, `fs/`, `http/`, `os/`, `types/`, `vec/`) **and** the appropriate generation directory (`v1/` or `v2/`).

3. **`v1` preserves original behavior.** Do not change `v1` signatures or semantics unless you are fixing an actual bug. Bugfixes in `v1` are allowed only if they do not break existing users (document any unavoidable breakage).

4. **`v2` is the improved API.** You may change signatures, return `Result`, introduce typed errors, split/merge functions — as long as the set of operations available in `v1` remains achievable in `v2`.

5. **One file per logical operation.** Do not put the whole module implementation into `mod.rs`. Split files by purpose:
   - `copy.rs`, `write.rs`, `normalize.rs`, `create_dir.rs`, `exists.rs`, `current_dir.rs`, `expand.rs`, `remove_dir_all.rs`, `copy_dir_all.rs`, `test.rs` — filesystem primitives;
   - `config/` — subdirectory with `error.rs`, `load.rs`, `get.rs`, `set.rs`, `save.rs`, `paths.rs`, `file.rs`, `get_value.rs`, `mod.rs`;
   - `client.rs` — shared HTTP client;
   - `error.rs` — typed errors;
   - `os.rs`, `arch.rs`, `detect.rs` — enum + detection split;
   - `line.rs` (single file) or `line/` (directory) — pick one form per generation, do not mix.

6. **Features are additive in Cargo.** If a user enables both `fs-v1` and `fs-v2`, both compile flags become active. Conflicts must be caught at compile time via `compile_error!` in `<mod>/mod.rs` (see snippet above).

7. **Modules are optional.** Do not add `compile_error!` for "no version of this module enabled". If neither `fs-v1` nor `fs-v2` is on, `pub mod fs` simply does not exist (gated in `lib.rs` via `#[cfg(any(feature = "fs-v1", feature = "fs-v2"))]`).

8. **Inter-module dependencies** are checked in `lib.rs`. Current contracts:
   - `fs-v1` → requires `types-v1` or `types-v2`
   - `types-v1` / `types-v2` → requires `color-v1` or `color-v2`
   - `http-v1` → requires `fs-v1`/`fs-v2` and `types-v1`/`types-v2`
   - `http-v2` → self-contained (no `fs` dependency)

   If you add a new cross-module dependency, add a matching `compile_error!` in `lib.rs`.

9. **Do not break `available_modules()`.** It reports the modules actually compiled into the current build, driven by features via `#[cfg]`. Do not hardcode a list that can drift from reality. `tests/versions.rs` verifies that the reported list matches the enabled features.

10. **No unnecessary dependencies.** Prefer `std`; add crates only when they are genuinely useful.

11. **Every change comes with tests.** This is required, not optional:
    - **New public item** (function, method, type, enum variant, field, macro) — add tests that cover normal behavior, boundary values, and error/edge cases where they exist.
    - **Changed behavior or signature** — update the existing tests that assert on the old behavior, and add new ones for the new behavior.
    - **Bug fix** — add a regression test that fails without the fix and passes with it.
    - **New module or generation** — a dedicated `tests/<module>.rs` (or `<module>_v<N>_*.rs` for larger modules) with a `#[cfg(feature = "...")]` gate at the top, matching the existing layout in `tests/`.

    Place tests in `tests/` when exercising the public API. Use an inline `#[cfg(test)] mod tests` inside a source file only when the code under test is private (`pub(crate)` / private) and cannot be reached from an integration test.

    Tests must be deterministic, independent, safe for the filesystem (use `tests/common::TempDir`, never real user paths), and resilient to parallel execution. Do not hard-code network access — use the in-process test server from `tests/common`.

12. **Every public change is reflected in the documentation.** This is also required:
    - **New or changed public item** — update the corresponding page under `docs/pages/modules/<module>/<generation>.md` (and/or `docs/pages/lib.md` for crate-root items). Describe the new function/type, its parameters, return value, error conditions, and limitations. Add a usage example if it helps.
    - **Changes that alter the "differences from the other generation"** — update the comparison table on the affected page.
    - **New module or generation** — add the page(s) under `docs/pages/modules/<module>/`, add the module to the table in `docs/pages/modules/index.md`, and register the pages in the `nav:` section of `mkdocs.yml`.
    - **Changes to features, inter-module dependencies, or setup** — update `docs/pages/getting-started/installation.md` and/or `docs/pages/getting-started/features.md`.
    - **Rustdoc** — every new `pub` item gets a `///` doc comment explaining its purpose; new modules get a `//!` module-level comment.

    Documentation is part of the change. A PR that adds a public API without updating `docs/` is considered incomplete.

13. **Run the checks before pushing:**
    ```bash
    make fmt-check
    make clippy
    make test
    ```

    Or, in a single step:

    ```bash
    make check
    ```

    `cargo test --all-features` does **not** work for this crate: `v1` and `v2` generations of a module are mutually exclusive, and enabling both triggers `compile_error!`. Always test one generation at a time — the `Makefile` does this for you.

    If you touched documentation, also verify the MkDocs site builds without warnings:

    ```bash
    venv/bin/mkdocs build --strict
    # or, for a live preview during editing:
    venv/bin/mkdocs serve
    ```

14. By submitting a Pull Request, you agree that your code will be distributed under the **MPL-2.0** license.

---

Questions and suggestions — in Issues.

---

## RU

Спасибо за интерес к проекту!

### Структура

Библиотека использует **независимое версионирование модулей**. Каждый модуль живёт в своём каталоге с двумя поколениями рядом:

```
src/
├── lib.rs              # гейтинг модулей + available_modules() + проверки межмодульных зависимостей
├── types_macros.rs     # макрос log! (доступен всегда, #[macro_export])
├── color/  { mod.rs, v1/, v2/ }
├── fs/     { mod.rs, v1/, v2/ }
├── http/   { mod.rs, v1/, v2/ }
├── os/     { mod.rs, v1/, v2/ }
├── types/  { mod.rs, v1/, v2/ }
└── vec/    { mod.rs, v1/, v2/ }
tests/                  # интеграционные тесты, один файл на модуль (и на поколение)
├── common/             # хелперы (TempDir, тестовый HTTP-сервер, env-lock)
├── color.rs
├── fs_v1_basic.rs
├── fs_v1_config.rs
├── fs_v2_basic.rs
├── fs_v2_config.rs
├── http_v1.rs
├── http_v2.rs
├── macros.rs
├── os.rs
├── types.rs
├── vec.rs
└── versions.rs         # интеграционные тесты available_modules()
docs/                   # исходники сайта MkDocs
├── index.md            # главная страница сайта
├── README.md           # страница для GitHub
└── pages/
    ├── getting-started/
    ├── lib.md
    └── modules/<module>/<generation>.md
mkdocs.yml              # конфигурация MkDocs и навигация
Makefile                # make test / make lint / make check
```

`src/<mod>/mod.rs` — только объявления и реэкспорт:

```rust
#[cfg(all(feature = "fs-v1", feature = "fs-v2"))]
compile_error!("features `fs-v1` and `fs-v2` are mutually exclusive; enable exactly one");

#[cfg(feature = "fs-v1")]
pub mod v1;
#[cfg(feature = "fs-v2")]
pub mod v2;

#[cfg(feature = "fs-v1")]
pub use self::v1::*;
#[cfg(all(feature = "fs-v2", not(feature = "fs-v1")))]
pub use self::v2::*;
```

Имена фич: `<модуль>-<поколение>` — `fs-v1`, `fs-v2`, `vec-v1`, `vec-v2`, `http-v1`, `http-v2`, `color-v1`, `color-v2`, `os-v1`, `os-v2`, `types-v1`, `types-v2`. `default = []` — по умолчанию не включён ни один модуль.

### Правила

1. **Все изменения отправляются в ветку `dev`.** Pull Request открывается против `dev`, а не `main`.

2. **Сохраняйте модульную структуру.** Новый функционал идёт в соответствующий модуль (`color/`, `fs/`, `http/`, `os/`, `types/`, `vec/`) **и** в правильное поколение (`v1/` или `v2/`).

3. **`v1` сохраняет исходное поведение.** Не меняйте сигнатуры и семантику `v1` без необходимости. Багфиксы в `v1` допустимы только если не ломают существующих пользователей (несовместимость, если неизбежна, должна быть задокументирована).

4. **`v2` — улучшенный API.** Можно менять сигнатуры, возвращать `Result`, вводить типизированные ошибки, разделять/объединять функции — при условии, что набор операций `v1` остаётся выполнимым в `v2`.

5. **Один файл — одна логическая операция.** Не складывайте всю реализацию модуля в `mod.rs`. Дробите файлы по назначению:
   - `copy.rs`, `write.rs`, `normalize.rs`, `create_dir.rs`, `exists.rs`, `current_dir.rs`, `expand.rs`, `remove_dir_all.rs`, `copy_dir_all.rs`, `test.rs` — примитивы файловой системы;
   - `config/` — подкаталог с `error.rs`, `load.rs`, `get.rs`, `set.rs`, `save.rs`, `paths.rs`, `file.rs`, `get_value.rs`, `mod.rs`;
   - `client.rs` — общий HTTP-клиент;
   - `error.rs` — типизированные ошибки;
   - `os.rs`, `arch.rs`, `detect.rs` — разделение enum и определения;
   - `line.rs` (один файл) или `line/` (каталог) — выберите одну форму на поколение, не смешивайте.

6. **Features в Cargo аддитивны.** Если пользователь включит `fs-v1` и `fs-v2` одновременно, оба флага станут активными. Конфликты должны ловиться на этапе компиляции через `compile_error!` в `<mod>/mod.rs` (см. сниппет выше).

7. **Модули необязательны.** Не добавляйте `compile_error!` на «не включена ни одна версия модуля». Если не включены ни `fs-v1`, ни `fs-v2` — `pub mod fs` просто не существует (гейтинг в `lib.rs` через `#[cfg(any(feature = "fs-v1", feature = "fs-v2"))]`).

8. **Межмодульные зависимости** проверяются в `lib.rs`. Текущие контракты:
   - `fs-v1` → требует `types-v1` или `types-v2`
   - `types-v1` / `types-v2` → требует `color-v1` или `color-v2`
   - `http-v1` → требует `fs-v1`/`fs-v2` и `types-v1`/`types-v2`
   - `http-v2` → самодостаточен (не зависит от `fs`)

   Если добавляете новую межмодульную зависимость — добавьте соответствующий `compile_error!` в `lib.rs`.

9. **Не ломайте `available_modules()`.** Она отражает модули, реально вошедшие в текущую сборку, и управляется features через `#[cfg]`. Не хардкодьте список, который может разойтись с реальностью. `tests/versions.rs` проверяет, что выданный список соответствует включённым features.

10. **Никаких лишних зависимостей.** Предпочитайте `std`; добавляйте crates только при реальной пользе.

11. **Любое изменение сопровождается тестами.** Это обязательное требование, а не пожелание:
    - **Новый публичный элемент** (функция, метод, тип, вариант enum'а, поле, макрос) — добавьте тесты, покрывающие нормальное поведение, граничные значения и ошибки/крайние случаи, если они есть.
    - **Изменение поведения или сигнатуры** — обновите существующие тесты, проверяющие старое поведение, и добавьте новые под новое.
    - **Исправление бага** — добавьте регрессионный тест, который падает без исправления и проходит с ним.
    - **Новый модуль или поколение** — отдельный `tests/<module>.rs` (или `<module>_v<N>_*.rs` для крупных модулей) с `#[cfg(feature = "...")]` в начале файла, по образцу существующих файлов в `tests/`.

    Тесты, проверяющие публичный API, размещайте в `tests/`. Встроенный `#[cfg(test)] mod tests` внутри исходника используйте только когда тестируемый код приватный (`pub(crate)` / private) и недоступен из интеграционного теста.

    Тесты должны быть детерминированными, независимыми друг от друга, безопасными для файловой системы (используйте `tests/common::TempDir`, никаких реальных пользовательских путей) и устойчивыми к параллельному запуску. Никаких обращений к настоящей сети — используйте встроенный тестовый сервер из `tests/common`.

12. **Любое публичное изменение отражается в документации.** Это тоже обязательное требование:
    - **Новый или изменённый публичный элемент** — обновите соответствующую страницу в `docs/pages/modules/<module>/<generation>.md` (или `docs/pages/lib.md` для элементов на корне крейта). Опишите новую функцию/тип, её параметры, возвращаемое значение, условия ошибок и ограничения. Добавьте пример использования, если он помогает.
    - **Изменения в разделе «differences from the other generation»** — обновите таблицу сравнения на затронутой странице.
    - **Новый модуль или поколение** — добавьте страницу(ы) в `docs/pages/modules/<module>/`, добавьте модуль в таблицу в `docs/pages/modules/index.md` и зарегистрируйте страницы в секции `nav:` файла `mkdocs.yml`.
    - **Изменения в фичах, межмодульных зависимостях или установке** — обновите `docs/pages/getting-started/installation.md` и/или `docs/pages/getting-started/features.md`.
    - **Rustdoc** — каждый новый `pub`-элемент получает комментарий `///` с описанием назначения; новые модули получают комментарий уровня модуля `//!`.

    Документация — часть изменения. PR, который добавляет публичный API, но не трогает `docs/`, считается незавершённым.

13. **Перед пушем прогоняйте проверки:**
    ```bash
    make fmt-check
    make clippy
    make test
    ```

    Либо одной командой:

    ```bash
    make check
    ```

    `cargo test --all-features` для этого крейта **не работает**: поколения `v1` и `v2` одного модуля взаимоисключающие, их одновременное включение вызывает `compile_error!`. Всегда тестируйте одну генерацию за раз — `Makefile` делает это за вас.

    Если вы меняли документацию, дополнительно проверьте, что сайт MkDocs собирается без предупреждений:

    ```bash
    venv/bin/mkdocs build --strict
    # или, для живого предпросмотра при редактировании:
    venv/bin/mkdocs serve
    ```

14. Отправляя Pull Request, вы соглашаетесь, что ваш код будет распространяться под лицензией **MPL-2.0**.

---

Вопросы и предложения — в Issues.