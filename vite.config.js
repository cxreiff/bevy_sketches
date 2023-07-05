import { defineConfig } from 'vite'
import ViteRsw from 'vite-plugin-rsw'

export default defineConfig({
  base: "/bevy_sketches/",
  assetsInclude: [
    './crates/*/assets/**/*',
    './crates/*/assets/*',
    './crates/**/assets/*',
    './crates/sketch_1/assets/*',
    './crates/sketch_1/assets/**/*',
    'crates/sketch_1/assets/**',
    'crates/sketch_1/assets/**/*',
    'crates/sketch_1/assets/**/*.png',
    './crates/sketch_1/assets/**/*.png',
    '/crates/sketch_1/assets/**/*.png',
  ],
  plugins: [
    ViteRsw(),
  ],
})
