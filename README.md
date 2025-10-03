<div align="center">
    <a href="https://elcharitas.wtf"><h1 align="center">elcharitas.wtf</h1></a>
    
My personal website, built with [Rust](https://www.rust-lang.org/) using [Axum](https://github.com/tokio-rs/axum) and [Tower](https://github.com/tower-rs/tower), styled with [Tailwind CSS](https://tailwindcss.com/). The site uses [Momenta](https://github.com/elcharitas/momenta) for UI components, [Datastar](https://data-star.dev/) for reactivity, and integrates with GitHub API for projects and Hashnode for blog posts.

</div>

## Running Locally

```sh-session
git clone https://github.com/elcharitas/elcharitas.wtf.git
cd elcharitas.wtf
```

Create a `.env` file similar to [`.env.example`](https://github.com/elcharitas/elcharitas.wtf/blob/main/.env.example).

Then install Tailwind CSS:

```sh-session
npm install -g tailwindcss@3
```

and run the development server:

```sh-session
cargo run
```

For development with CSS hot-reloading, you can use cargo-watch:

```sh-session
cargo watch -x run
```

## Cloning / Forking

Please remove all of my personal information (projects, images, etc.) before deploying your own version of this site.
