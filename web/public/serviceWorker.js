importScripts("/lib/localforage.min.js");
importScripts("/lib/locks.js");
importScripts("/lib/worker.js");
importScripts("/lib/worker-init.js");
importScripts("/lib/message.js");

const ACCESS_TOKEN = "application-access-token";
const ACCESS_TOKEN_CONFIRMED_UNAUTHORIZED = "unauthorized";

let refreshTokensLock = new ExecutionLock();

// self.addEventListener("fetch", (event) => event.respondWith(logFetch(event)));
self.addEventListener("fetch", async (event) => {
    event.respondWith(processFetch(event));
    // event.respondWith(logFetch(event));
});

self.addEventListener("message", async (event) => {
    console.debug("sw: message", event);
    if (event.data.type === "login-callback") {
        const token = event.data.token;
        console.info("sw: received token from callback page", token);

        // Store the token in for later use
        await localforage.setItem(ACCESS_TOKEN, token);

        // Redirect the page to the home page (removes the token from the url)
        const redirectTo = event.data.redirectTo;
        const client = event.source; 
        // TODO: Should we check to see if the client is a window?
        client.navigate(redirectTo);
    } else if (event.data.type === "logout") {
        // Remove the token from localForage
        await localforage.removeItem(ACCESS_TOKEN);

        const client = event.source;
        const redirectTo = event.data.redirectTo;
        client.navigate(redirectTo);
    }
});

function processFetch(event) {
    const url_string = event.request.url;
    const url = new URL(url_string);
    console.debug("sw: checking", url);
    if (url.pathname.includes("api")) {
        console.log("has api");
    }

    // If the request is navigating to a new page, or is an API request, then we want to allow it through unmodified
    const urlIsThisHost = url.hostname === self.location.hostname;
    const isApiRequest = event.request.mode === "navigate" || url.pathname.startsWith("/api/");
    if (urlIsThisHost && isApiRequest) {
        // This is a reuqest to the API, so we need to ensure there is a token available
        return processTokenFetch(event);
    }

    console.debug("sw: skipping", url);
    return fetch(event.request);
}

async function processTokenFetch(event) {
    const url = new URL(event.request.url);
    console.log("sw: processing", url);

    /*
    * Goal: Attach an access token to requests that match the following criteria:
    * - The request is to the same host as the service worker
    * - The request is not a navigation request
    * - The request is going to the API (route starts with /api/)
    */

    /*
    * Retrieving a token:
    * - Ideally, we want to use the token that is stored in localForage
    * - If there is no token in localForage, then we want to request the token from the page
    * - Finally, if there is no token in localForage and the page doesn't have a token, then we want to request a new token from the server
    * - If we don't have a token at this point, then we are unauthorized and should let the request through without a token
    */

    await refreshTokensLock.waitForUnlock();

    let accessToken = await localforage.getItem(ACCESS_TOKEN);

    if (accessToken == null) {
        // If there isn't a token, try to refresh
        accessToken = await refreshToken(event);

        // If we can't refresh (token is null), then we are unauthorized
        if (accessToken == null) {
            accessToken = ACCESS_TOKEN_CONFIRMED_UNAUTHORIZED;
            await localforage.setItem(ACCESS_TOKEN, accessToken);
        }
    }

    // finally, if we have a token, then we can add it to the request
    // Check if the token is not expired
    if (
        typeof accessToken === "string" &&
        accessToken !== ACCESS_TOKEN_CONFIRMED_UNAUTHORIZED
    ) {
        let expiration = await (async () => {
            let decodedBody = JSON.parse(atob(accessToken.split(".")[1]));
            return decodedBody.exp;
        })();

        // If the token is expired, then refresh it
        if (expiration < getTimestampSeconds()) {
            accessToken = await refreshToken(event);
        }

        // If the token is a string and not "unauthorized", then we are authorized
        if (
            typeof accessToken === "string" &&
            accessToken !== ACCESS_TOKEN_CONFIRMED_UNAUTHORIZED
        ) {
            const modifiedRequest = addAuthHeader(
                event.request,
                accessToken
            );

            return fetch(modifiedRequest);
        }
    }

    // if we get here, we should return unmodified. Ideally we should redirect to the login page
    return fetch(event.request);
}

/* Attempt to refresh the token
*   - if the request is successful, the access token is stored in localForage
*/
async function refreshToken(event) {
    // TODO: We don't support refreshing tokens yet
    return null;
}
