import { fileURLToPath, URL } from 'node:url'
import { resolve } from 'node:path';

import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'


const __dirname = fileURLToPath(new URL('.', import.meta.url));

export default defineConfig({
  plugins: [
    vue(),
  ],
  resolve: {
    alias: {
      '@app': resolve(__dirname, 'src'),
      '@components': resolve(__dirname, 'src/components'),
      '@common': resolve(__dirname, 'src/common'),
      '@wallet': resolve(__dirname, 'src/components/wallet'),
      '@stores': resolve(__dirname, 'src/stores'),
      '@views': resolve(__dirname, 'src/views')
    }
  }
})
