const API_HOST = new URL("https://ding.ecko.ga");

async function get(url, init) {
    const response = await fetch(new URL(url, API_HOST), {
        method: 'GET',
        ...init
    });
    return {
        status: response.status,
        body: await response.json(),
    };    
}

async function post(url, body = null, init = {}) {
    const params = {
        method: 'POST',
        ...init
    };
    if (body != null) {
        params.body = JSON.stringify(body);
    }
    const response = await fetch(new URL(url, API_HOST), params);
    return {
        status: response.status,
        body: await response.json(),
    };
}

export default {
    get,
    post
}