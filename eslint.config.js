import antfu from '@antfu/eslint-config'

export default antfu({
  unocss: true,
  ignores: [
    '**/src-tauri/**',
    'src/vite-env.d.ts',
  ],
  rules: {
    'node/prefer-global/process': 'off',
    'no-console': 'off',
  },
})
