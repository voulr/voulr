<p align="center">
  <p align="center">
   <img width="150" height="150" src="apps/web/src/lib/assets/pngs/voulr-app-logo.png" alt="logo">
  </p>
	<h1 align="center"><b>Voulr</b></h1>
	<p align="center">
        A Lambda platform for backend developers
    <br />
    <a href="https://voulr.com"><strong>voulr.com »</strong></a>
    <br />
</p>

Voulr is an open source Lambda platform written in rust.

## Architecture

-   `web`: A static site built with [Svelte](https://svelte.dev) and deployed to [Cloudflare Pages](https://pages.cloudflare.com).
-   `server`: A rust server built with [Axum](https://github.com/tokio-rs/axum) deployed to [AWS Lambda](https://aws.amazon.com/lambda).

## Credits

-   Design inspired by [Scale](https://scale.com) and [Stripe](https://stripe.com).
-   Idea inspired by [Cloudflare Pages](https://pages.cloudflare.com) and [Cargo Lambda](https://cargo-lambda.info).
