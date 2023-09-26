import init, { on_message, on_fetch } from "/wasm/service-worker/annapurna_service_worker.js";

console.log("start: sw-module.js");

self.addEventListener("install", (event) => {
    // go directly from "installed" to "activated" even if there are precious instances of the service worker
    console.debug("Service worker installed");
    event.waitUntil(self.skipWaiting());
});

self.addEventListener("activate", (event) => {
    // Claim all clients immediately
    // This ensures that page is controlled without waiting for a reload
    console.debug("Service worker activated");
    event.waitUntil(self.clients.claim());
});

self.addEventListener("message", async (event) => {
    await init();

    on_message(event);
});

self.addEventListener("fetch", async (event) => {
    await init();

    event.respondWith(on_fetch(event));
});

console.log("end: sw-module.js");
