import init, { on_message, on_fetch } from "/js/service-worker/annapurna_service_worker.js";

self.addEventListener("install", (event) => {
    // go directly from "installed" to "activated" even if there are previous instances of the service worker
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
    console.debug("message event");
    await init();

    await on_message(event);
});

self.addEventListener("fetch", async (event) => {
    console.debug("fetch event");

    const fetch_func = async () => {
        await init();
        const response = await on_fetch(event);
        return response;
    }

    event.respondWith(fetch_func());
});
