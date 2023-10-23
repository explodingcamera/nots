# Nots - Not Serverless (Coming Soon)

> Lightweigth orchestration for the stateless web.<br>
> Perfect for eveything from a one-off script to your next big thing.

A single binary that transforms your linux server(s) into the best way to deploy anything web.

Use the `nots` cli to deploy
- Code in any git repository
- A tarball from your CI server
- A Docker image
- `a_randome_one_of_project.ts` you just wrote

and have it running in seconds and updated automatically without any downtime.

## Usage

### On your server
```bash
[you@yourserver]$ curl -fsSL https://nots.dev/install | bash

# start the nots server
# this will guide you through the setup process
[you@yourserver]$ nots server init
> What port should the server listen on? 8080
> Should the server restart automatically on reboot? [y/N] y
> Open advanced settings? [y/N] N
> Your server is now running on port 8080

[you@yourserver]$ nots deploy https://github.com/example/my-project.git --name my-project
> What host should this service be available at? localhost:8080
> What path should this service be available at? /api
> Do you want to do any advanced configuration now? [y/N] N
> Your service is now available at http://localhost:8080/api
# nots will now automatically pull the latest version of your code and restart the service
# and route all traffic from https://my-project.example.com/api to your service
```

### or on your local machine
```bash
# write some code on your local machine and deploy it instantly
# a simple hello world with bun
[you@localhost]$ echo "export default { fetch: req => new Response('hi'), port: process.env.PORT }}" > hi.ts

# deploy remotely to your server
[you@localhost]$ nots --ssh you@yourserver deploy ./hi.ts --name hi --engine bun
```

## Supported Runtimes
- [ ] Docker Containers - anything that can run with `docker run`
- [ ] Binaries - just provide a url to a binary and Nots will download and run it as a service
- [ ] Bun - everything from a serverless function to a full backend service