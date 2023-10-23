export default {
  fetch: async () => {
    await new Promise((resolve) => setTimeout(resolve, 10))
    new Response('hi')
  },
  port: 3333,
}