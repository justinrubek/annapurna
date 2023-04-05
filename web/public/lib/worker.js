/// serviceworker fetch handler that logs the request url
function logFetch(event) {
    const url = new URL(event.request.url);
    console.log("fetch", url);

    return fetch(event.request);
}

/// Appends the authorization header to the request with the access token
function addAuthHeader(request, accessToken) {
    console.log("adding auth header", accessToken);
    const modifiedHeaders = new Headers(request.headers);
    modifiedHeaders.append("Authorization", `Bearer ${accessToken}`);

    const modifiedRequestInit = { headers: modifiedHeaders, mode: "same-origin" };

    return new Request(request, modifiedRequestInit);
}

function matchUrls(url, urls = [], containsUrls = []) {
    const matchUrls = urls.find((element) => {
        if (url.pathname.includes(element)) {
            return true;
        }
    });

    const matchContainsUrls = containsUrls.find((element) => {
        if (url.pathname.includes(element)) {
            return true;
        }
    });

    return matchUrls || matchContainsUrls;
}

function getTimestampSeconds() {
    return Math.floor(Date.now() / 1000);
}

function isLoginCallback(url) {
    const urls = [
        "/login-callback",
    ];
    console.log("checking if login callback", url);

    return matchUrls(url, urls);
}
