// These listeners will install the service worker and activate it immediately.
// Import them in your service worker file to use them.

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
