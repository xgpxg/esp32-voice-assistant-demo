import {defineConfig, loadEnv} from "vite";
import vue from "@vitejs/plugin-vue";
import {fileURLToPath, URL} from 'node:url'
import {createSvgIconsPlugin} from 'vite-plugin-svg-icons'

// https://vite.dev/config/
export default defineConfig(async ({mode}) => {
    const root = process.cwd();
    const viteEnv = loadEnv(mode, root);
    const PROXY_SERVER = viteEnv.VITE_PROXY_SERVER
    return {
        plugins: [vue(),
            createSvgIconsPlugin({
                // Specify the icon folder to be cached
                iconDirs: [fileURLToPath(new URL('./src/icons', import.meta.url))],
                // Specify symbolId format
                symbolId: 'icon-[dir]-[name]',
            }),
        ],
        resolve: {
            alias: {
                '@': fileURLToPath(new URL('./src', import.meta.url)),
                "@components": fileURLToPath(new URL('./src/components', import.meta.url)),
                "@images": fileURLToPath(new URL('./src/assets/images', import.meta.url)),
            }
        },
        server: {
            port: 9010,
            strictPort: true,
            host: true,
            allowedHosts: [
                'aiway.node-1.bala',
                'localhost',
                '127.0.0.1'
            ],
            proxy: {
                '/api/': {
                    target: PROXY_SERVER,
                    changeOrigin: true,
                    rewrite: (path) => path.replace(/^\/api/, "/")
                },
                '/file/': {
                    target: PROXY_SERVER,
                    changeOrigin: true,
                },
            },
        },
    }
});
