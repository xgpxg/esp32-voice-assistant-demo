const BASE_URL = '';

const API = {
    BASE_URL,
    WIFI_LIST: `${BASE_URL}/api/wifi/list`,
    WIFI_CONNECT: `${BASE_URL}/api/wifi/connect`,
    WIFI_IS_CONNECTED: `${BASE_URL}/api/wifi/is_connected`,
}

const checkResponse = (response) => {
    if (response.code !== 0) {
        msg.error(response.msg);
        throw new Error(response.msg || '未知错误');
    }
    return response.data;
};

const getWifiList = () => {
    return fetch(API.WIFI_LIST).then(res => res.json()).then(checkResponse)
}

const connectWifi = (ssid, password) => {
    return fetch(API.WIFI_CONNECT, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({
            ssid,
            password
        })
    }).then(res => res.json())
        .then(checkResponse);
}

const wifiIsConnected = () => {
    return fetch(API.WIFI_IS_CONNECTED).then(res => res.json()).then(checkResponse)
}
