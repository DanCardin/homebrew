import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import path from 'path'

export default defineConfig({
  plugins: [vue()],
  resolve: {
    alias: {
      '@': path.resolve(__dirname, '/src'),
    },
  },
  server: {
    port: 6272,
    open: true,
    proxy: {
      "/api": {
        target: process.env.API_URL || "http://127.0.0.1:6273",
        rewrite: (path) => path.replace(/^\/api/, ""),
      },
    },
  },
})
