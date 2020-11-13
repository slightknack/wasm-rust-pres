// All fetch events will be handled by our program
addEventListener('fetch', event => {
  event.respondWith(handleEvent(event))
})

async function handleEvent(event) {
    // Bind our Rust WASM library to this program
    await wasm_bindgen(wasm)

    // Get the main function from that library
    const { main } = wasm_bindgen;

    // Call out to Rust/WASM, have it handle the event
    // Returns some html
    const content = await main(event.request);

    // returns some html
    return new Response(content, {
        headers: { "content-type": "text/html; charset=utf-8" },
        status: 200
    })
}
