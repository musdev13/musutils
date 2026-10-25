# Contributing to musutils / Вклад в musutils

---

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
tests/
└── versions.rs         # integration tests for available_modules()
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

1. **Keep the modular structure.** New functionality goes into the appropriate module (`color/`, `fs/`, `http/`, `os/`, `types/`, `vec/`) **and** the appropriate generation directory (`v1/` or `v2/`).

2. **`v1` preserves original behavior.** Do not change `v1` signatures or semantics unless you are fixing an actual bug. Bugfixes in `v1` are allowed only if they do not break existing users (document any unavoidable breakage).

3. **`v2` is the improved API.** You may change signatures, return `Result`, introduce typed errors, split/merge functions — as long as the set of operations available in `v1` remains achievable in `v2`.

4. **One file per logical operation.** Do not put the whole module implementation into `mod.rs`. Split files by purpose:
   - `copy.rs`, `write.rs`, `normalize.rs`, `create_dir.rs`, `exists.rs`, `current_dir.rs`, `expand.rs`, `remove_dir_all.rs`, `copy_dir_all.rs`, `test.rs` — filesystem primitives;
   - `config/` — subdirectory with `error.rs`, `load.rs`, `get.rs`, `set.rs`, `save.rs`, `paths.rs`, `file.rs`, `get_value.rs`, `mod.rs`;
   - `client.rs` — shared HTTP client;
   - `error.rs` — typed errors;
   - `os.rs`, `arch.rs`, `detect.rs` — enum + detection split;
   - `line.rs` (single file) or `line/` (directory) — pick one form per generation, do not mix.

5. **Features are additive in Cargo.** If a user enables both `fs-v1` and `fs-v2`, both compile flags become active. Conflicts must be caught at compile time via `compile_error!` in `<mod>/mod.rs` (see snippet above).

6. **Modules are optional.** Do not add `compile_error!` for "no version of this module enabled". If neither `fs-v1` nor `fs-v2` is on, `pub mod fs` simply does not exist (gated in `lib.rs` via `#[cfg(any(feature = "fs-v1", feature = "fs-v2"))]`).

7. **Inter-module dependencies** are checked in `lib.rs`. Current contracts:
   - `fs-v1` → requires `types-v1` or `types-v2`
   - `types-v1` / `types-v2` → requires `color-v1` or `color-v2`
   - `http-v1` → requires `fs-v1`/`fs-v2` and `types-v1`/`types-v2`
   - `http-v2` → self-contained (no `fs` dependency)
   
   If you add a new cross-module dependency, add a matching `compile_error!` in `lib.rs`.

8. **Do not break `available_modules()`.** It reports the modules actually compiled into the current build, driven by features via `#[cfg]`. Do not hardcode a list that can drift from reality. `tests/versions.rs` verifies that the reported list matches the enabled features.

9. **No unnecessary dependencies.** Prefer `std`; add crates only when they are genuinely useful.

10. **Run the checks before pushing:**
    ```bash
    cargo fmt --check
    cargo check --no-default-features --features "color-v1,types-v1,fs-v1,vec-v1,os-v1,http-v1"
    cargo check --no-default-features --features "color-v2,types-v2,fs-v2,vec-v2,os-v2,http-v2"
    cargo check --no-default-features --features "fs-v2,vec-v1,http-v2,color-v1,os-v2,types-v2"
    cargo test
    cargo clippy --all-targets --all-features
    ```

11. By submitting a Pull Request, you agree that your code will be distributed under the **MPL-2.0** license.

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
tests/
└── versions.rs         # интеграционные тесты available_modules()
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

1. **Сохраняйте модульную структуру.** Новый функционал идёт в соответствующий модуль (`color/`, `fs/`, `http/`, `os/`, `types/`, `vec/`) **и** в правильное поколение (`v1/` или `v2/`).

2. **`v1` сохраняет исходное поведение.** Не меняйте сигнатуры и семантику `v1` без необходимости. Багфиксы в `v1` допустимы только если не ломают существующих пользователей (несовместимость, если неизбежна, должна быть задокументирована).

3. **`v2` — улучшенный API.** Можно менять сигнатуры, возвращать `Result`, вводить типизированные ошибки, разделять/объединять функции — при условии, что набор операций `v1` остаётся выполнимым в `v2`.

4. **Один файл — одна логическая операция.** Не складывайте всю реализацию модуля в `mod.rs`. Дробите файлы по назначению:
   - `copy.rs`, `write.rs`, `normalize.rs`, `create_dir.rs`, `exists.rs`, `current_dir.rs`, `expand.rs`, `remove_dir_all.rs`, `copy_dir_all.rs`, `test.rs` — примитивы файловой системы;
   - `config/` — подкаталог с `error.rs`, `load.rs`, `get.rs`, `set.rs`, `save.rs`, `paths.rs`, `file.rs`, `get_value.rs`, `mod.rs`;
   - `client.rs` — общий HTTP-клиент;
   - `error.rs` — типизированные ошибки;
   - `os.rs`, `arch.rs`, `detect.rs` — разделение enum и определения;
   - `line.rs` (один файл) или `line/` (каталог) — выберите одну форму на поколение, не смешивайте.

5. **Features в Cargo аддитивны.** Если пользователь включит `fs-v1` и `fs-v2` одновременно, оба флага станут активными. Конфликты должны ловиться на этапе компиляции через `compile_error!` в `<mod>/mod.rs` (см. сниппет выше).

6. **Модули необязательны.** Не добавляйте `compile_error!` на «не включена ни одна версия модуля». Если не включены ни `fs-v1`, ни `fs-v2` — `pub mod fs` просто не существует (гейтинг в `lib.rs` через `#[cfg(any(feature = "fs-v1", feature = "fs-v2"))]`).

7. **Межмодульные зависимости** проверяются в `lib.rs`. Текущие контракты:
   - `fs-v1` → требует `types-v1` или `types-v2`
   - `types-v1` / `types-v2` → требует `color-v1` или `color-v2`
   - `http-v1` → требует `fs-v1`/`fs-v2` и `types-v1`/`types-v2`
   - `http-v2` → самодостаточен (не зависит от `fs`)
   
   Если добавляете новую межмодульную зависимость — добавьте соответствующий `compile_error!` в `lib.rs`.

8. **Не ломайте `available_modules()`.** Она отражает модули, реально вошедшие в текущую сборку, и управляется features через `#[cfg]`. Не хардкодьте список, который может разойтись с реальностью. `tests/versions.rs` проверяет, что выданный список соответствует включённым features.

9. **Никаких лишних зависимостей.** Предпочитайте `std`; добавляйте crates только при реальной пользе.

10. **Перед пушем прогоняйте проверки:**
    ```bash
    cargo fmt --check
    cargo check --no-default-features --features "color-v1,types-v1,fs-v1,vec-v1,os-v1,http-v1"
    cargo check --no-default-features --features "color-v2,types-v2,fs-v2,vec-v2,os-v2,http-v2"
    cargo check --no-default-features --features "fs-v2,vec-v1,http-v2,color-v1,os-v2,types-v2"
    cargo test
    cargo clippy --all-targets --all-features
    ```

11. Отправляя Pull Request, вы соглашаетесь, что ваш код будет распространяться под лицензией **MPL-2.0**.

---

Вопросы и предложения — в Issues.