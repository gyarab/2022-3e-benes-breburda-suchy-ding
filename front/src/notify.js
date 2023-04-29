import { reactive } from 'vue'

const notifications = reactive([])

function send(type, text, timeout = 3000) {
    const id = Math.random();
    notifications.push({
        id,
        type,
        text
    })

    setTimeout(() => {
        // remove notification if it still exists
        const idx = notifications.findIndex(i => i.id == id);
        idx < 0 || notifications.splice(idx, 1);
    }, timeout)
}

function capture_err(resp) {
    if (resp.status != 200) {
        send('error', resp.body.message);
        throw new Error(`API returned code ${resp.status}`);
    }
    return resp;
}

export {
    notifications,
    send as notify,
    capture_err
}
