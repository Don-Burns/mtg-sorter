import path from "path";
import { defineConfig } from "vite";
import solid from "vite-plugin-solid";

export default defineConfig({
  plugins: [solid()],
  // set base  url for assets, default is '/'
  // base: "/static",
  build: {
    minify: false,
    // generate manifest.json
    manifest: true,
    // specify build directory, default is dist
    // outDir: "dist/",
    // set entrypoints for components, can be single entrypoint for entire app
    // default is index.html and index.js in frontend if this option doesn't set
    rollupOptions: {
      input: {
        index: path.resolve(__dirname, "index.html"),
        upload_success: path.resolve(__dirname, "upload_success.html"),
        technology: path.resolve(__dirname, "technology.html"),
      },
    },
  },
});
