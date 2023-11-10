<div align="center">
  <h1 align="center"> Nots - Beyond Serverless 🌟</h1>
  <a href="https://github.com/yourrepo/nots"><img src="https://img.shields.io/badge/Status-Coming%20Soon-yellow.svg" alt="Coming Soon"></a>
  <a href="https://opensource.org/licenses/MIT"><img src="https://img.shields.io/badge/License-MIT-green.svg" alt="MIT License"></a>
  <a href="https://github.com/explodingcamera/nots/releases"><img alt="GitHub release (with filter)" src="https://img.shields.io/github/v/release/explodingcamera/nots?filter=nots-cli*&style=social"></a>
</div>

<br>

> [!WARNING]  
> Nots is currently in development. The CLI is available for testing, but most features are not yet implemented.

**Nots** is a tool that takes the hassle out of deploying your apps. You bring your code, and `nots` deploys it using the best-suited runtime — be it Bun, Docker, or a standalone binary. It manages your secrets and environment variables, keeping them secure. Plus, it smartly routes traffic to your apps. And you can focus on building great software without relying on proprietary cloud infrastructure. 

## 🛠 Usage

### On Your Server

<pre><code><i># Download and install Nots</i>
$ <b>curl</b> -fsSLO https://nots.dev/install.sh
$ <b>chmod</b> +x install.sh && ./install.sh

<i># Install the nots server</i>
$ <b>nots</b> server init
</code></pre>

### Local Machine

<pre><code><i># Write and deploy code instantly; here's a basic "Hello World" example using Bun</i>
$ <b>echo</b> "export default { fetch: req => new Response('hi'), port: process.env.PORT }" > hi.ts

<i># Deploy remotely to your server</i>
$ <b>nots</b> app create demo-app --runtime=bun --route="api.example.com/hi/*"
$ <b>nots</b> app deploy demo-app ./hi.ts

<i># Deploy from you CI</i>
$ <b>cargo</b> build --release --bin app
$ <b>nots</b> app deploy app ./target/release/app
</code></pre>

## 🏗 Supported Runtimes

- **Bin**: Static Binaries. No need to build a new Docker Image for every app version.
- **Bun**: Run TypeScript/JavaScript Code - no build step required

with more to come soon

## License

Nots is licensed under the terms of both the MIT License and the Apache License (Version 2.0).
See [LICENSE-APACHE](./LICENSE-APACHE) and [LICENSE-MIT](./LICENSE-MIT) for details.
Any contribution intentionally submitted for inclusion in Nots shall be dual licensed as above without any additional terms or conditions.