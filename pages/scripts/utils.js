let messageContainer = null;

/**
 * 显示消息提示
 * @param {string} text - 要显示的消息文本
 * @param {string} type - 消息类型: 'info', 'success', 'warning', 'error'
 * @param {number} duration - 显示持续时间(毫秒)，为0则不自动消失
 */
function showMessage(text, type = 'info', duration = 3000) {
    // 创建消息容器（如果不存在）
    if (!messageContainer) {
        messageContainer = document.createElement('div');
        messageContainer.id = 'message-container';
        messageContainer.style.cssText = `
        position: fixed;
        top: 20px;
        left: 50%;
        transform: translateX(-50%);
        z-index: 9999;
        max-width: 300px;
        margin: 0 auto;
        `;
        document.body.appendChild(messageContainer);
    }

    const msgEl = document.createElement('div');
    let bgColor = '#333';

    switch (type) {
        case 'success':
            bgColor = '#4CAF50';
            break;
        case 'warning':
            bgColor = '#FF9800';
            break;
        case 'error':
            bgColor = '#F44336';
            break;
        default:
            bgColor = '#2196F3';
    }

    msgEl.style.cssText = `
        background-color: ${bgColor};
        color: white;
        padding: 12px 16px;
        margin-bottom: 10px;
        border-radius: 4px;
        box-shadow: 0 2px 8px rgba(0,0,0,0.2);
        font-size: 14px;
        opacity: 0;
        transform: translateY(-10px);
        transition: all 0.3s ease;
  `;

    msgEl.textContent = text;
    messageContainer.appendChild(msgEl);

    setTimeout(() => {
        msgEl.style.opacity = '1';
        msgEl.style.transform = 'translateY(0)';
    }, 10);

    if (duration > 0) {
        setTimeout(() => {
            msgEl.style.opacity = '0';
            msgEl.style.transform = 'translateY(-10px)';

            setTimeout(() => {
                if (msgEl.parentNode) {
                    msgEl.parentNode.removeChild(msgEl);

                    // 如果没有更多消息，则移除容器
                    if (messageContainer && messageContainer.children.length === 0) {
                        messageContainer.parentNode.removeChild(messageContainer);
                        messageContainer = null;
                    }
                }
            }, 300);
        }, duration);
    }

    return msgEl;
}

const msg = {
    info: (text, duration) => showMessage(text, 'info', duration),
    success: (text, duration) => showMessage(text, 'success', duration),
    warning: (text, duration) => showMessage(text, 'warning', duration),
    error: (text, duration) => showMessage(text, 'error', duration),
    hide: () => {
        if (messageContainer) {
            document.body.removeChild(messageContainer);
            messageContainer = null;
        }
    }
};

function alert(message) {
    return showMessage(message, 'info', 3000);
}

if (typeof module !== 'undefined' && module.exports) {
    module.exports = {showMessage, alert, msg};
} else {
    window.showMessage = showMessage;
    window.alert = alert;
    window.msg = msg;
}