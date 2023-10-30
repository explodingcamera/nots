<h1 align="center"> Nots - Beyond Serverless 🌟</h1>

<p align="center">
  <a href="https://github.com/yourrepo/nots"><img src="https://img.shields.io/badge/Status-Coming%20Soon-yellow.svg" alt="Coming Soon"></a>
  <a href="https://opensource.org/licenses/MIT"><img src="https://img.shields.io/badge/License-MIT-green.svg" alt="MIT License"></a>
  <a href="https://github.com/explodingcamera/nots/releases"><img alt="GitHub release (with filter)" src="https://img.shields.io/github/v/release/explodingcamera/nots?filter=nots-cli*&style=social"></a>
</p>

<br>

<p align="center">
    🚀 <strong>Deploy Stateless Apps with Ease</strong><br>
    🛠 <strong>Ideal for everything from quick scripts to large-scale projects</strong>
</p>

<br>

With `nots`, you can deploy:
- Code from any Git repository
- Tarballs generated by your CI server
- Docker images
- That fresh `a_random_project.ts` you just coded

Get up and running in seconds, with zero downtime during updates.

## 🛠 Usage
### On Your Server
<pre><code><i># Download and install Nots (Soon!)</i>
$ <b>curl</b> -fsSL https://nots.dev/install | <b>bash</b>

<i># Initialize the nots server</i>
$ <b>nots</b> server init
</code></pre>

### Local Machine
<pre><code><i># Write and deploy code instantly; here's a basic "Hello World" example using Bun</i>
$ <b>echo</b> "export default { fetch: req => new Response('hi'), port: process.env.PORT }" > hi.ts

<i># Deploy remotely to your server via SSH or the http API</i>
$ <b>nots</b> --ssh you@yourserver deploy ./hi.ts --name hi --engine bun
</code></pre>

## 🏗 Supported Runtimes
- **Docker Containers:** anything that can run with `docker run`
- **Binaries:** Provide a URL and Nots downloads and runs it as a service
- **Bun:** From serverless functions to full-fledged backend services
- more coming soon

## Why
* Automatic Reverse Proxy to your services, with SSL and HTTP/2 support.
* Building new Docker images for every commit is overkill; lockfiles exist for a reason. Run code without rebuilding entire environments, and without the long CI build times.
* Current open-source FaaS/Serverless solutions are lacking. Skip the new framework learning curve.
* Sometimes, simplicity is key. All you might need is a single-file script.
* Not every project demands colossal scalability. Often, a humble SQLite database and a couple of endpoints suffice.

## License

Nots is licensed under the [Apache 2.0 License](./LICENSE). All Copyrights are retained by their Contributors.