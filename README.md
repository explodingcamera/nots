# Nots - Not Serverless (Coming Soon)

> Lightweigth orchestration for your web services<br/>
> Perfect for eveything from a one-off script to your next big thing.

A single binary that transforms your linux server into the best way to deploy anything web.

Use the `nots` commandline tool to deploy
- Code in any git repository
- A Docker image
- a_randome_one_of_project.ts you just wrote

```bash
echo "Bun.serve({ fetch: req => new Response('Bun!') }})" > index.ts
nots deploy index.ts --name bun --host bun.example.com --path /hello --engine bun
```

and have it running in seconds and updated automatically.

## Supported Runtimes
- [ ] Docker Container - anything that can run with `docker run`
- [ ] Binaries - just provide a url to a binary and Nots will download and run it as a service
- [ ] Bun - everything from a serverless function to a full backend service