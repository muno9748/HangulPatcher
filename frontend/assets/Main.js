const noKeySupport = (() => {
    // <span class="InvalidKeyDialog">해당 키는 지원되지 않습니다!</span>

    const span = document.createElement('span')

    span.classList.add('InvalidKeyDialog')
    span.innerText = '해당 키는 지원되지 않습니다!'

    return span
})()
const usingNativeMode = key => key == 0x15
const keyMap = new Map([
    ['LShift', 0xA0],
    ['RShift', 0xA1],

    ['A', 0x41],
    ['B', 0x42],
    ['C', 0x43],
    ['D', 0x44],
    ['E', 0x45],
    ['F', 0x46],
    ['G', 0x47],
    ['H', 0x48],
    ['I', 0x49],
    ['J', 0x4A],
    ['K', 0x4B],
    ['L', 0x4C],
    ['M', 0x4D],
    ['N', 0x4E],
    ['O', 0x4F],
    ['P', 0x50],
    ['Q', 0x51],
    ['R', 0x52],
    ['S', 0x53],
    ['T', 0x54],
    ['U', 0x55],
    ['V', 0x56],
    ['W', 0x57],
    ['X', 0x58],
    ['Y', 0x59],
    ['Z', 0x5A],

    ['F1', 0x70],
    ['F2', 0x71],
    ['F3', 0x72],
    ['F4', 0x73],
    ['F5', 0x74],
    ['F6', 0x75],
    ['F7', 0x76],
    ['F8', 0x77],
    ['F9', 0x78],
    ['F10', 0x79],
    ['F11', 0x7A],
    ['F12', 0x7B],

    ['Backquote', 0xC0],
    ['Escape', 0x1B],

    ['0', 0x30],
    ['1', 0x31],
    ['2', 0x32],
    ['3', 0x33],
    ['4', 0x34],
    ['5', 0x35],
    ['6', 0x36],
    ['7', 0x37],
    ['8', 0x38],
    ['9', 0x39],

    ['Enter', 0x0D],
    ['Space', 0x20],
    ['Tab', 0x09],
    ['Backspace', 0x08],
    ['Backslash', 0xE2],
    ['Minus', 0xBD],
    ['Equal', 0xBB],
    ['Comma', 0xBC],
    ['Period', 0xBE],
    ['Slash', 0xBF],
])

const reverseKeyMap = new Map(Array.from(keyMap.entries()).map(([k, v]) => [v, k]))

function storageProxy() {
    return new Proxy({
        ChatOpenKey: 0,
        KoreanToggleKey: 0,
        KeepKorean: false,
        BlockKoreanInGame: false,
        OverlayEnabled: false
    }, {
        get(_, key) {
            switch (key) {
                case 'ChatOpenKey':
                case 'KoreanToggleKey': 
                    return +localStorage.getItem(key)
                case 'KeepKorean':
                case 'BlockKoreanInGame':
                case 'OverlayEnabled':
                    return !!+localStorage.getItem(key)
                default: throw new Error('Invalid Key')
            }
        },
        set(_, key, v) {
            switch (key) {
                case 'ChatOpenKey': {
                    __TAURI__.event.emit('set_setting', `0,${v}`)
                    localStorage.setItem(key, String(v))
                    break
                }
                case 'KoreanToggleKey': { 
                    __TAURI__.event.emit('set_setting', `1,${v}`)
                    localStorage.setItem(key, String(v))
                    break
                }
                case 'BlockKoreanInGame': {
                    __TAURI__.event.emit('set_setting', `2,${+v}`)
                    localStorage.setItem(key, String(+v))
                    break
                }
                case 'KeepKorean': {
                    __TAURI__.event.emit('set_setting', `3,${+!v}`)
                    localStorage.setItem(key, String(+v))
                    break
                }
                case 'OverlayEnabled': {
                    __TAURI__.event.emit('set_setting', `4,${+v}`)
                    localStorage.setItem(key, String(+v))
                    break
                }
                default: throw new Error('Invalid Key')
            }
        }
    })
}

function createDefaultSettings() {
    localStorage.setItem('ChatOpenKey', 0x54.toString())
    localStorage.setItem('KoreanToggleKey', 0x15.toString())
    localStorage.setItem('KeepKorean', '1')
    localStorage.setItem('BlockKoreanInGame', '1')
    localStorage.setItem('OverlayEnabled', '1')
    localStorage.setItem('SettingsLoaded', '1')
}

function getStorage() {
    if(!+localStorage.getItem('SettingsLoaded')) createDefaultSettings()

    const storage = storageProxy()

    __TAURI__.event.emit('load', [storage.ChatOpenKey, storage.KoreanToggleKey, +storage.BlockKoreanInGame, +!storage.KeepKorean, +storage.OverlayEnabled].join(','))

    return storage
}

const currentVersion = '3.0.0'

fetch('https://api.github.com/repos/muno9748/HangulPatcher/releases/latest').then(resp => resp.json()).then(async resp => {
    if(resp.tag_name != currentVersion) {
        const resp = await __TAURI__.dialog.ask('새로운 버전이 있습니다.\n다운받으시겠습니까?', 'Korean Patcher')

        if(resp) __TAURI__.shell.open('https://github.com/muno9748/HangulPatcher/releases/latest')
    }
})

const storage = getStorage()
document.querySelector('.start').addEventListener('click', e => {
    if(e.target.classList.contains('pause')) {
        e.target.classList.remove('pause')
    } else {
        e.target.classList.add('pause')
    }
})

document.querySelector('.exit').addEventListener('click', () => {
    __TAURI__.window.getCurrent().close()
})

document.querySelector('.github').addEventListener('click', () => {
    __TAURI__.shell.open('https://github.com/muno9748/HangulPatcher')
})

document.querySelector('.SetChatOpenKeyText').innerText = reverseKeyMap.get(storage.ChatOpenKey)
document.querySelector('#NativeMode').checked = usingNativeMode(storage.KoreanToggleKey)
document.querySelector('#CtrlMode').checked = !usingNativeMode(storage.KoreanToggleKey)
document.querySelector('#BlockKorean').checked = storage.BlockKoreanInGame
document.querySelector('#KeepKorean').checked = storage.KeepKorean
document.querySelector('#InGameOverlay').checked = storage.OverlayEnabled

let isRecordingKey = false

document.querySelector('.SetChatOpenKey').addEventListener('click', () => {
    if(!isRecordingKey) {
        isRecordingKey = true

        document.querySelector('.SetChatOpenKeyModalContainer').classList.add('Recording')
    }
})

function setChatOpenKeyText(text) {
    const el = document.querySelector('.SetChatOpenKeyText')

    el.classList.add('KeyTextDown')
    
    setTimeout(() => {
        el.innerText = text
        el.classList.remove('KeyTextDown')
    }, 300)
}

document.addEventListener('keydown', e => {
    if (!isRecordingKey) return

    e.preventDefault()

    let result = e.code

    if (result == ' ') {
        result = 'Space'
    } else if (result.includes('Shift')) {
        result = ['L', 'R'][e.location - 1] + 'Shift'
    } else if (result.startsWith('Key') && result.length == 4) {
        result = result.slice(3)
    } else if (result.startsWith('Digit') && result.length == 6) {
        result = result.slice(5)
    }

    if (!keyMap.has(result)) {
        document.querySelector('.SetChatOpenKeyModalContainer').appendChild(noKeySupport)
        return
    }
    
    storage.ChatOpenKey = keyMap.get(result)

    isRecordingKey = false
    document.querySelector('.SetChatOpenKeyModalContainer').classList.remove('Recording')

    setChatOpenKeyText(result)
})

document.querySelector('#NativeMode').addEventListener('change', e => {
    if(e.target.checked) storage.KoreanToggleKey = 0x15
})

document.querySelector('#CtrlMode').addEventListener('change', e => {
    if(e.target.checked) storage.KoreanToggleKey = 0xA2
})

document.querySelector('#BlockKorean').addEventListener('change', e => {
    const { checked } = e.target

    storage.BlockKoreanInGame = checked
})

document.querySelector('#KeepKorean').addEventListener('change', e => {
    const { checked } = e.target

    storage.KeepKorean = checked
})

document.querySelector('#InGameOverlay').addEventListener('change', e => {
    const { checked } = e.target

    storage.OverlayEnabled = checked
})

let isRunning = true

document.querySelector('.start').addEventListener('click', () => {
    isRunning = !isRunning

    if(isRunning) {
        __TAURI__.event.emit('run')
    } else {
        __TAURI__.event.emit('stop')
    }
})