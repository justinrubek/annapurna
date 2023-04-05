
/// Use this function to send a message to a controlled page.
/// The message will request that the page send the value of the access token to the service worker.
/// If the page has a valid access token, it will be returned through another message.
const requestFromPage = async () => {
    // use any controlled page as the client
    const allClients = await self.clients.matchAll();
    const client = allClients.filter(client => client.type === "window")[0];

    // We can't send a message to a client that doesn't exist, so return null
    if (client == null) {
        return null;
    }

    // We're going to use a MessageChannel to communicate
    const channel = MessageChannel();

    console.debug("requesting token from page");
    client.postMessage({
        type: "request-token",
    }, [channel.port1]);

    // wrap the onmessage handler in a promise
    return new Promise((resolve, reject) => {
        channel.port2.onmessage = (event) => {
            if (event.data.error) {
                console.error("error requesting token from page", event.error);
                reject(event.data.error);
            }

            resolve(event.data.token);
        };
    });
};
