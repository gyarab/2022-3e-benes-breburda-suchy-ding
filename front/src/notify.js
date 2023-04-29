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

export default {
    notifications,
    send
}
