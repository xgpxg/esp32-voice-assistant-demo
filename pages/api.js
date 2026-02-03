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

/**
 * 获取WiFi列表
 * @returns {Promise<any>}
 */
const getWifiList = () => {
    return fetch(API.WIFI_LIST).then(res => res.json()).then(checkResponse)
}

/**
 * 连接WiFi
 * @param ssid WIFI名
 * @param password WIFI密码
 * @returns {Promise<{ssid: string, signal_strength: number, auth_method: string}>}
 */
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

/**
 * 判断WiFi是否已连接。
 * 已连接：返回连接的SSID
 * 未连接：返回null
 * @returns {Promise<any>}
 */
const wifiIsConnected = () => {
    return fetch(API.WIFI_IS_CONNECTED).then(res => res.json()).then(checkResponse)
}
