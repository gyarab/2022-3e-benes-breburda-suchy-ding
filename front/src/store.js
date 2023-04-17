import { reactive } from "vue";
import rest from './rest';

const store = reactive({
    user: null,
});

async function loadUserData() {
    const resp = await rest.get('/api/users/me');
    if (resp.status != 200) {
        return;
    }
    store.user = resp.body;
    store.loggedIn = true;
}

export {
    store,
    loadUserData,
};