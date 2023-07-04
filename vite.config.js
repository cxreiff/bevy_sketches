import { defineConfig } from 'vite'
import ViteRsw from 'vite-plugin-rsw'

export default defineConfig({
  base: "/bevy_sketches/",
  plugins: [
    ViteRsw(),
  ],
})
