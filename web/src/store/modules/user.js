const state = {
    isLogin: false,
    token: '',
    username: '',
}

//初始化
const initState = (state, token, username) => {
    if (token) {
        state.token = token
        state.isLogin = true
        state.username = username
    }
}

initState(state, localStorage.getItem('token'), localStorage.getItem('username'))


const mutations = {
    setToken: (state, token) => {
        state.token = token
        state.isLogin = true
        localStorage.setItem('token', token)
    },
    setUsername: (state, username) => {
        state.username = username
        localStorage.setItem('username', username)
    },

    resetToken: (state) => {
        state.isLogin = false
        state.token = ''
        state.username = ''
        localStorage.removeItem('token')
        localStorage.removeItem('debug_app')
    },
}

const actions = {}

export default {
    namespaced: true,
    state,
    mutations,
    actions
}
