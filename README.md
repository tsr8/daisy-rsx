## Daisy-RSX

This is a [Dioxus](https://dioxuslabs.com/) version of the [Daisy UI](https://daisyui.com/) components.

## Installation
1. Add to Cargo.toml:

```
[dependencies]
daisy_rsx = { git = "https://github.com/tsr8/daisy-rsx.git", branch = "daisy-5.0.6" }
...
```

2. Create input.css in main directory:

```
@import "tailwindcss";
@plugin "daisyui";
```
3. Install TailwindCSS:

```sh
npm install tailwindcss
```

4. Install DaisyUI:

```sh
npm install daisyui

```

5. Run in terminal:

```sh
npx @tailwindcss/cli -i ./input.css -o ./assets/tailwind.css --watch
```
