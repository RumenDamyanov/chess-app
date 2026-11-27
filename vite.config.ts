import react from '@vitejs/plugin-react';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { defineConfig } from 'vite';

const root = path.dirname(fileURLToPath(import.meta.url));
const host = process.env.TAURI_DEV_HOST;

export default defineConfig(() => ({
  plugins: [react()],
  clearScreen: false,
  resolve: {
    alias: {
      '@': path.resolve(root, 'src'),
      '@components': path.resolve(root, 'src/components'),
      '@hooks': path.resolve(root, 'src/hooks'),
      '@services': path.resolve(root, 'src/services'),
      '@types': path.resolve(root, 'src/types'),
      '@utils': path.resolve(root, 'src/utils'),
      '@styles': path.resolve(root, 'src/styles'),
      '@contexts': path.resolve(root, 'src/contexts'),
    },
  },
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: 'ws' as const,
          host,
          port: 1421,
        }
      : undefined,
    watch: {
      ignored: ['**/src-tauri/**'],
    },
  },
}));
