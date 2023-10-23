$$ \text{\Huge Nots - Beyond Serverless 🌟} $$

<p align="center">
  <a href="https://github.com/yourrepo/nots"><img src="https://img.shields.io/badge/Status-Coming%20Soon-yellow.svg" alt="Coming Soon"></a>
  <a href="https://opensource.org/licenses/MIT"><img src="https://img.shields.io/badge/License-MIT-green.svg" alt="MIT License"></a>
</p>

<blockquote style="border-left: 4px solid #ccc; padding-left: 1em;">
  🚀 <strong>Streamlined Orchestration for Stateless Applications</strong><br>
  🛠 <strong>Ideal for everything from quick scripts to large-scale projects</strong>
</blockquote>

<br>

With `nots`, deploy:
- Code from any Git repository
- Tarballs generated by your CI server
- Docker images
- That fresh `a_random_project.ts` you just coded

Get up and running in seconds, with zero downtime during updates.

## 🛠 Usage
### On Your Server
<pre><code><i># Download and install Nots</i>
$ <b>curl</b> -fsSL https://nots.dev/install | <b>bash</b>

<i># Initialize the nots server</i>
$ <b>nots</b> server init
</code></pre>

### Local Machine
<pre><code><i># Write and deploy code instantly; here's a basic "Hello World" example using Bun</i>
$ <b>echo</b> "export default { fetch: req => new Response('hi'), port: process.env.PORT }}" > hi.ts

<i># Deploy remotely to your server via SSH or the http API</i>
$ <b>nots</b> --ssh you@yourserver deploy ./hi.ts --name hi --engine bun
</code></pre>

## 🏗 Supported Runtimes
- Docker Containers: anything that can run with `docker run`
- Binaries: Provide a URL and Nots downloads and runs it as a service
- Bun: From serverless functions to full-fledged backend services
- more coming soon

## Why
* Docker images are overkill; lockfiles exist for a reason. Run code without rebuilding entire environments.
* Current open-source FaaS/Serverless solutions are lacking. Skip the new framework learning curve.
* Sometimes, a single-file script is all you need.
* Most projects don't require massive scale. A simple SQLite database and a few endpoints usually suffice.