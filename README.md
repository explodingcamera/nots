<img src="./nots.jpg" />
<p align="center">
  <br/>
    <a href="https://nots.dev">Nots</a> is a self-hosted cloud platform —
    <br/>
    Just bring your code, and Nots will take care of the rest.
</p>
<br>

> [!WARNING]  
> Nots is currently in development. The CLI is available for testing, but most features are not yet implemented. See the [roadmap](./ROADMAP.md) for more information.

With **Nots**, you can transform any server into a powerful, scalable, and secure cloud platform. It's a self-hosted alternative to serverless and edge platforms like AWS Lambda, Google Cloud Functions, Vercel, and Cloudflare Workers. You bring your code, and `nots` deploys it using the best-suited runtime — be it Bun, Docker, or a standalone binary.
The focus is on simplicity: You provide an artifact, and `nots` takes care of the rest. Unlike other platforms, `nots` doesn't require you to build a new Docker image for every app version, and it doesn't force you to use a specific language or framework. Additionally, it manages your secrets and environment variables, keeping them secure. Plus, it smartly routes traffic to your apps.

All of this without any "magic" - no special runtime or framework is required to run your apps.
 
## 📖 Table of Contents

- [📖 Table of Contents](#-table-of-contents)
- [🛠 Installation](#-installation)
- [🚀 Getting Started](#-getting-started)
  - [Installing the Server Daemon](#installing-the-server-daemon)
  - [📡 Connecting to the Server Daemon](#-connecting-to-the-server-daemon)
  - [💻 Creating an App](#-creating-an-app)
  - [📦 Deploying an Artifact](#-deploying-an-artifact)
  - [📈 Scaling your Servers](#-scaling-your-servers)
  - [❄️ Cold Boots/Hot Boots](#️-cold-bootshot-boots)
- [🏗 Supported Runtimes](#-supported-runtimes)
- [📝 Roadmap](#-roadmap)
- [📄 License](#-license)

## 🛠 Installation
<pre><code>$ <b>curl</b> -fsSLO https://nots.dev/install.sh
$ <b>chmod</b> +x install.sh && ./install.sh
</code></pre>

The `nots` CLI is the primary way to interact with the `nots` platform. It's a single binary that you can use to deploy, manage, and monitor your apps. it is available for Linux, macOS, and Windows.

Before installing, make sure you have [Docker](https://docs.docker.com/get-docker/) installed on your machine, as it's currently the only backend supported by `nots` (Firecracker based runtimes are coming soon).

Alternatively, you can download the latest binary from the [releases page](https://github.com/explodingcamera/nots/releases). The installation script just downloads the latest binary and places it in `~/.local/bin` (You can als read the script before running it to make sure it's safe).

## 🚀 Getting Started

### Installing the Server Daemon

Every `nots` installation requires a server daemon. It's a small binary that runs on your server and manages your apps. You can install it by running the following command:

<pre><code>$ <b>nots server init</b></code></pre>

This will guide you through the installation process. You can also use the `--help` flag to see all available options.

### 📡 Connecting to the Server Daemon


The `nots` CLI automatically connects to the server daemon running on your machine. Remote connections are not yet supported, but they're coming soon.

<!-- If you want to connect to a remote server, set the `NOTS_SERVER` environment variable to the server's address. For example:

<pre><code>$ <b>export NOTS_SERVER=ssh://user@server</b></code></pre> -->

### 💻 Creating an App

> [!WARNING]
> This feature is not yet implemented.

Apps are the primary unit of deployment in `nots`. By default, all apps are in the same, global namespace, but you will soon be able to organize them into different projects, like a separate namespace for your personal projects and another one for your company's apps.

You can create a new app by running the following command:

<pre><code>$ <b>nots app create</b></code></pre>

### 📦 Deploying an Artifact

> [!WARNING]
> This feature is not yet implemented.

Once you've created an app, you can deploy an artifact to it. An artifact is a binary or any archive that contains your app's code. You can deploy an artifact by running the following command:

<pre><code>$ <b>nots deploy --app=example-app ./index.ts</b>    <i># Single Files</i>
$ <b>nots deploy --app=example-app ./app.tar.gz</b>  <i># Archives</i>
$ <b>nots deploy --app=example-app ./dist</b>        <i># Directories</i>
</code></pre>

### 📈 Scaling your Servers

> [!WARNING]
> This feature is not yet implemented.

Currently, `nots` only supports a single server, but you will soon be able to scale your apps across multiple servers and even multiple regions. Nots however will not balance requests between servers - load balancing can be done on the DNS level, or you can use CDN providers like Cloudflare to route traffic to the closest server. Simplicity is key - most projects will never need a complex load balancing setup and simple and efficient code can handle a lot more traffic than you might think.

### ❄️ Cold Boots/Hot Boots

By default, `nots` keeps your apps running indefinitely. However, you will soon be able to configure it to shut down your apps after a certain period of inactivity. This is useful for apps that are only used occasionally, like small side projects. The time it takes to boot up an app is heavily dependent on the runtime. For example, a Rust binary takes a few milliseconds to start, while a Node.js app can take up to a few seconds.
To reduce this for slow runtimes, `nots` will also support hot boots using cgroup freezer. This will allow you to keep your apps running indefinitely while saving cpu resources when they're not in use.

## 🏗 Supported Runtimes

`nots` supports essentially any programming language or framework.
We provide a set of official runtimes, but you can use any OCI image as a runtime.
Nots just creates a VM or container for a given OCI image, and then runs your app inside it (and provides some additional features like automatic routing, secrets, and more).

Official runtimes (optimized for performance and security):
- [Bun](https://github.com/nots-dev/runtimes#bun) 
- [Node.js](https://github.com/nots-dev/runtimes#node)
- [Deno](https://github.com/nots-dev/runtimes#deno)
- [Binary](https://github.com/nots-dev/runtimes) - any standalone binary
- Docker (you don't even need a 'runtime' for this, runtimes are just OCI images)

A NotsTs runtime based on Bun is also in the works, which will offer a [WinterCG](https://wintercg.org/) based runtime for TypeScript and JavaScript.

You can also create your own runtime based on the existing ones. Check out the [runtimes](https://github.com/nots-dev/runtimes) repository for more information.

## 📝 Roadmap

The current roadmap is available [here](./ROADMAP.md).

<!-- ## 📚 Documentation

* CLI
* Continuous Deployment
* App Configuration
  * Secrets
  * Environment Variables

## 📖 Cookbook

* [JavaScript/TypeScript](https://nots.dev/cookbook/js)
  * [Next.js](https://nots.dev/cookbook/js/nextjs)
  * [Express](https://nots.dev/cookbook/js/express)
  * [Hono](https://nots.dev/cookbook/hono)
* [Rust](https://nots.dev/cookbook/rust)
* [Go](https://nots.dev/cookbook/go) -->

## 📄 License

Nots is licensed under the terms of both the MIT License and the Apache License (Version 2.0).
See [LICENSE-APACHE](./LICENSE-APACHE) and [LICENSE-MIT](./LICENSE-MIT) for details.
Any contribution intentionally submitted for inclusion in Nots shall be dual licensed as above without any additional terms or conditions.