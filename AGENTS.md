# HID Tester

Tauri 2 + hidapi. Identifier `com.gestaobem.hid-tester`. Frontend vanilla em `web/`. Parser HID em `src-tauri/src/boot_report.rs`.

Um comando de teste: `npm test` (na pasta `produtos/hid-tester`). CI: `npm run ci` (Prettier, ESLint, testes). Pre-commit husky: `lint-staged` + `npm test`.

Log: `/tmp/hid-tester.jsonl` (append, um JSON por linha: `inventory`, `open`, `report`, `raw`, `error`).
