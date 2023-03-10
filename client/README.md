# Sveltenest - birdnest client

Frontend implementation for Reaktor's [Summer Assignment](https://assignments.reaktor.com/birdnest), utilizing the custom Rust backend.

Live version deployed at [https://birdrust.siidorow.com](https://birdrust.siidorow.com)!

## Developing

Once you've created a project and installed dependencies with `pnpm install` and copied the `.env.example` to `.env`, start a development server:

```bash
pnpm run dev

# or start the server and open the app in a new browser tab
pnpm run dev -- --open
```

## Building

To create a production version of your app:

```bash
pnpm run build
```

You can preview the production build with `pnpm run preview`.

> To deploy your app, you may need to install an [adapter](https://kit.svelte.dev/docs/adapters) for your target environment.
