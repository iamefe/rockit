# Rockit: A simple todo app built with Rust & SvelteKit in Tauri

## Author

Ativie Oserefemhen

## Tauri + SvelteKit + Tailwind CSS + TypeScript + Sqlite

This template should help get you started developing with Tauri, SvelteKit and TypeScript in Vite.

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

## Tips

### Use snake_case for Rust commands and camelcase for the corresponding Svelte command handlers or just add a rename directive to your commands like so:

```rust
#[tauri::command(rename_all = "snake_case")]
```

### Tailwind CSS can be tricky to install

First, add Tailwind CSS and its dependencies:

```bash
yarn add tailwindcss postcss autoprefixer

```

Next, initialize Tailwind CSS configuration files:

```bash
npx tailwindcss init -p

```

This will create two configuration files: `tailwind.config.js` and `postcss.config.js`.

Update your `tailwind.config.js` to include your template paths:

```javascript
/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    './src/**/*.{html,js,svelte,ts}'
  ],
  theme: {
    extend: {},
  },
  plugins: [],
}

```

Create a `src/app.css file` and add the Tailwind directives:

```css
@tailwind base;
@tailwind components;
@tailwind utilities;

```

Ensure your `postcss.config.js` is set up to use Tailwind CSS:

```javascript
module.exports = {
  plugins: {
    tailwindcss: {},
    autoprefixer: {},
  }
}
```

Run your project:

```bash
yarn dev

```

### .taurignore

Create one to prevent reload loops due to database changes.

```javascript
tasks.db
```
