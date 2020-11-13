addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request))
})

async function handleRequest(request) {
    await wasm_bindgen(wasm)
    const { main } = wasm_bindgen;

    const content = main(request)

    return new Response(content, {
        headers: { "content-type": "text/html; charset=utf-8" },
        status: 200
    })
}
