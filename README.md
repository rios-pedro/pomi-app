# Pomi 🍙

Um timer Pomodoro minimalista para macOS, inspirado no Onigiri. Vive na barra de menu, sem ícone no Dock, sem distração.

## Stack

- [Tauri 2](https://tauri.app/) — Rust + WebView nativo
- React + TypeScript
- `tauri-plugin-positioner` — ancora a janela perto do ícone da tray
- `tauri-plugin-notification` — notificações nativas do macOS

## Funcionalidades

- Ícone na menu bar mostrando o tempo restante em tempo real
- Sem ícone no Dock nem no Cmd+Tab (roda como `Accessory`)
- Presets rápidos (5, 15, 25, 45 min) + slider customizável
- Notificação nativa quando o tempo esgota
- Janela flutuante tipo popover, ancorada abaixo do ícone da tray

## Rodando localmente

```bash
npm install
npm run tauri dev
```

## Build de produção

```bash
npm run tauri build
```

O `.app` gerado fica em `src-tauri/target/release/bundle/macos/pomi.app`.

> **Nota:** notificações só funcionam corretamente no `.app` buildado (`tauri build`), não no modo dev (`tauri dev`) — o macOS não registra apps do dev server no Notification Center.

## Estrutura

src/ → frontend React/TS
src-tauri/src/ → backend Rust (tray, comandos, janela)
src-tauri/capabilities/ → permissões do Tauri (notification, positioner, etc.)

## Licença

MIT