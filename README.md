# HID Tester

Bancada para o Keychron K3 Pro (`3434:0230`). Lê o relatório HID cru (boot keyboard 8 bytes) e grava JSONL em `/tmp/hid-tester.jsonl`.

[![CI](https://github.com/puppe1990/hid-tester/actions/workflows/ci.yml/badge.svg)](https://github.com/puppe1990/hid-tester/actions/workflows/ci.yml)

## Comandos

```bash
cd produtos/hid-tester
npm test          # node --test + cargo test --lib
npm run lint
npm run format:check
npm run ci        # prettier + eslint + tests
npm run dev       # Tauri 2
```

Pre-commit: lint-staged (ESLint + Prettier) e `npm test`.

Na primeira abertura o macOS pede **Monitoramento de Entrada**. Sem isso a sonda não abre o teclado.

Teclas Y, H, Enter, 1 e F1 ficam com contorno no layout (as que falharam no tester web).
