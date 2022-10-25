const {sassPlugin, postcssModules} = require('esbuild-sass-plugin')
const {copy} = require('esbuild-plugin-copy')
const path = require('node:path')

// Generate CSS/JS Builds
require('esbuild')
    .build({
        entryPoints: ['assets/styles/application.scss', 'assets/scripts/application.js'],
        outdir: 'assets/builds',
        bundle: true,
        minify: true,
        loader: {
          '.woff': 'file',
          '.woff2': 'file',
          '.png': 'file',
        },
        plugins: [
          sassPlugin({
            type: 'css',
            includePaths: [
              path.resolve(__dirname, 'node_modules'),
            ]
          }),
          copy({
            assets: {
              from: ['./node_modules/govuk-frontend/govuk/assets/**/*'],
              to: [''],
            },
          }),
      ],
    })
    .then(() => console.log('⚡ Build complete! ⚡'))
    .catch(() => process.exit(1));
