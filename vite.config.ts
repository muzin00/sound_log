import { defineConfig } from 'vite';
import vue from '@vitejs/plugin-vue';

const host = process.env.TAURI_DEV_HOST;

export default defineConfig({
  plugins: [vue()],
  // Vite が Rust のエラーを不明瞭にするのを防止します
  clearScreen: false,
  server: {
    // Tauri は固定ポートで動作しているので、そのポートが見つからない場合機能しません
    strictPort: true,
    // Tauri が要求しているホストが設定されている場合は、それを使用します
    host: host || false,
    port: 5173,
  },
  // `envPrefix` の項目で始まる環境変数は `import.meta.env` を通じて Tauri のソース・コードから参照できます
  envPrefix: ['VITE_', 'TAURI_ENV_*'],
  build: {
    // Tauriは、Windows では Chromium を、macOS と Linux では WebKit を使用しています
    target:
      process.env.TAURI_ENV_PLATFORM == 'windows'
        ? 'chrome105'
        : 'safari13',
    // デバッグ・ビルドでは「ファイルの軽量化（ミニファイ）」をさせない
    minify: !process.env.TAURI_ENV_DEBUG ? 'esbuild' : false,
    // デバッグ・ビルドで「SourceMap」を作成します
    sourcemap: !!process.env.TAURI_ENV_DEBUG,
  },
});