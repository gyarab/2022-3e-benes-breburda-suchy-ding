import { reactive } from "vue";
import rest from './rest';
import { capture_err } from './notify'

const store = reactive({
    user: null,
});

let userCache = {};

async function loadUserData() {
    const resp = await rest.get('/api/users/me');
    if (resp.status != 200) {
        return;
    }
    store.user = resp.body;
    store.loggedIn = true;
}

async function getUser(user_id) {
    const populate = async () => {
        const resp = capture_err(await rest.get('/api/users/' + user_id))
        userCache[user_id] = resp.body
        return resp.body
    }

    return userCache[user_id] || await populate()
}

function flushUserCache() {
    userCache = {}
}

export {
    store,
    loadUserData,
    getUser,
    flushUserCache
};
