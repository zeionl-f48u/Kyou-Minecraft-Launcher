import { resolve } from 'node:path'
import vue from '@vitejs/plugin-vue'
import MotionResolver from 'motion-v/resolver'
import UnoCSS from 'unocss/vite'
import AutoImport from 'unplugin-auto-import/vite'
import { NaiveUiResolver } from 'unplugin-vue-components/resolvers'
import Components from 'unplugin-vue-components/vite'
import { defineConfig } from 'vite'
import VueRouter from 'vue-router/vite'

const host = process.env.TAURI_DEV_HOST

export default defineConfig(async () => ({
  plugins: [
    VueRouter({
      dts: 'src/route-map.d.ts',
      routesFolder: 'src/pages',
      exclude: ['**/components/**'],
    }),
    vue(),
    AutoImport({
      imports: [
        'vue',
        {
          'naive-ui': [
            'useDialog',
            'useMessage',
            'useNotification',
            'useLoadingBar',
          ],
        },
      ],
    }),
    Components({
      dts: true,
      resolvers: [NaiveUiResolver(), MotionResolver()],
    }),
    UnoCSS(),
  ],

  resolve: {
    alias: {
      '@': resolve(__dirname, 'src'),
      '@pages': resolve(__dirname, 'src/pages'),
      '@layouts': resolve(__dirname, 'src/layouts'),
      '@components': resolve(__dirname, 'src/components'),
      '@composables': resolve(__dirname, 'src/common/composables'),
    },
  },

  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: 'ws',
          host,
          port: 1421,
        }
      : undefined,
    watch: {
      ignored: ['**/src-tauri/**'],
    },
  },
}))
