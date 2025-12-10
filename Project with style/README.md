# BMI Calculator - Yew Application

A BMI (Body Mass Index) calculator built with Yew framework, featuring interactive sliders and real-time BMI calculation.

## Features

- ✨ Modern Rust web framework (Yew)
- 🎨 Beautiful gradient styling
- ⚡ High performance WebAssembly
- 📊 Interactive BMI calculator with sliders
- 🎯 Real-time BMI calculation and category display

## Prerequisites

- Rust (latest stable version)
- `trunk` - Install with: `cargo install trunk`

## Running the Application

1. Build and serve the application:
```bash
trunk serve
```

2. Open your browser and navigate to `http://localhost:8080`

## Building for Production

To build the optimized production version:

```bash
trunk build --release
```

Or use the build script:
```bash
# Linux/macOS
./build.sh

# Windows
build.bat
```

The output will be in the `dist` directory.

## Deployment

This application can be deployed to any static hosting provider. The `dist` directory contains all the files needed for deployment.

### Netlify

1. Push your code to a Git repository
2. Connect your repository to Netlify
3. Netlify will automatically detect the `netlify.toml` configuration
4. Build command: `trunk build --release`
5. Publish directory: `dist`

Or deploy manually:
```bash
trunk build --release
netlify deploy --prod --dir=dist
```

### Vercel

1. Push your code to a Git repository
2. Import your project to Vercel
3. Vercel will automatically detect the `vercel.json` configuration
4. Build command: `trunk build --release`
5. Output directory: `dist`

Or deploy manually:
```bash
trunk build --release
vercel --prod
```

### GitHub Pages

**Automated (Recommended):**
1. Push your code to a GitHub repository
2. Go to Settings → Pages
3. Enable GitHub Actions for deployment
4. The `.github/workflows/deploy.yml` workflow will automatically:
   - Build with relative paths (works for both project and user pages)
   - Deploy on push to main/master

**Manual Deployment:**
```bash
./fix-github-pages.sh
# Or: trunk build --release --public-url ./
# Then copy dist/* to your gh-pages branch
```

**Note:** Relative paths work for both project pages (`username.github.io/repo-name/`) and user pages (`username.github.io/`)

### Cloudflare Pages

1. Push your code to a Git repository
2. Connect your repository to Cloudflare Pages
3. Build command: `trunk build --release`
4. Build output directory: `dist`
5. Root directory: `/`

### Manual Deployment

For any static hosting provider:

1. Build the application:
   ```bash
   trunk build --release
   ```

2. Upload the contents of the `dist` directory to your hosting provider

3. Ensure your hosting provider supports:
   - SPA routing (redirects all routes to `index.html`)
   - Serving `.wasm` files with correct MIME type (`application/wasm`)

## Project Structure

```
yew_styles/
├── Cargo.toml              # Rust dependencies
├── Trunk.toml              # Trunk build configuration
├── index.html              # HTML template with styles
├── netlify.toml            # Netlify deployment configuration
├── vercel.json             # Vercel deployment configuration
├── _redirects              # SPA redirect rules
├── build.sh                # Build script (Linux/macOS)
├── build.bat               # Build script (Windows)
├── .github/
│   └── workflows/
│       └── deploy.yml      # GitHub Actions deployment workflow
├── src/
│   └── main.rs             # Main Yew application
└── README.md               # This file
```

## Technologies Used

- **Yew**: Rust framework for building web applications
- **WebAssembly**: For high-performance web applications
- **Trunk**: Build tool for Rust web applications
- **web-sys**: Web API bindings for Rust

