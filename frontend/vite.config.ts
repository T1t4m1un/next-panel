import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react-swc';
import { VitePWA } from 'vite-plugin-pwa';

// https://vite.dev/config/
export default defineConfig({
  plugins: [
    react(),
    VitePWA({
      registerType: 'autoUpdate',
      workbox: {
        clientsClaim: true,
        skipWaiting: true,
      },
      manifest: {
        name: 'Next Panel',
        description: 'A better panel app for NAS',
        theme_color: '#282C34',
        icons: [
          {
            src: '/__static/vite.svg',
            type: 'image/svg+xml',
            sizes: '192x192',
          },
          {
            src: '/__static/vite.svg',
            type: 'image/svg+xml',
            sizes: '512x512',
          },
        ],
      },
      injectRegister: 'inline',
      strategies: 'injectManifest',
      srcDir: 'src',
      filename: 'sw.ts',
      devOptions: {
        enabled: true,
      },
    }),
  ],
  base: '__static',
  build: {
    minify: 'terser',
    terserOptions: {
      compress: {
        drop_console: true,
        drop_debugger: true,
      },
      mangle: true,
    },
  },
  server: {
    proxy: {
      '/api': {
        target: 'http://localhost:3000/',
        rewrite: path => path.replace(/^\/__static/, ''),
      },
    },
  },
});
