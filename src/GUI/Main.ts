import Electron, { ipcRenderer } from 'electron'

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

let chatOpenKey: number = null!

function recordKey(e: KeyboardEvent) {
    e.preventDefault()

    const key = e.code

    let result = key

    if (key == ' ') {
        result = 'Space'
    } else if (key.includes('Shift')) {
        result = ['L', 'R'][e.location - 1] + 'Shift'
    } else if (key.startsWith('Key') && key.length == 4) {
        result = key.slice(3)
    } else if (key.startsWith('Digit') && key.length == 6) {
        result = key.slice(5)
    }

    if (!keyMap.has(result)) {
        document.querySelector<HTMLSpanElement>('.keyText')!.innerText = '...'
        document.querySelector<HTMLDivElement>('.error')!.classList.add('active')
        return
    }

    document.querySelector<HTMLSpanElement>('.keyText')!.innerText = result[0].toUpperCase() + result.slice(1)
    document.querySelector<HTMLDivElement>('.error')!.classList.remove('active')

    chatOpenKey = keyMap.get(result)!
}

document.querySelector('.Key')!.addEventListener('click', () => {
    if (document.querySelector<HTMLDivElement>('.start')!.classList.contains('pause')) {
        ipcRenderer.sendSync('Patcher::stopHook')
    }
    document.querySelector('.modalContainer')!.classList.toggle('enabled')
    document.addEventListener('keydown', recordKey)
})

document.querySelector('.ModalExit')!.addEventListener('click', () => {
    chatOpenKey = null!
    document.removeEventListener('keydown', recordKey)
    document.querySelector<HTMLSpanElement>('.keyText')!.innerText = '...'
    document.querySelector('.modalContainer')!.classList.remove('enabled')
    document.querySelector<HTMLDivElement>('.error')!.classList.remove('active')
    if (document.querySelector<HTMLDivElement>('.start')!.classList.contains('pause')) {
        ipcRenderer.sendSync('Patcher::startHook')
    }
})

document.querySelector('.ModalOkay')!.addEventListener('click', () => {
    if (chatOpenKey) {
        localStorage.setItem('ChatOpenVKCode', chatOpenKey.toString())
        ipcRenderer.sendSync('Patcher::setChatOpen', chatOpenKey)
        document.querySelector<HTMLButtonElement>('.Key')!.innerText = reverseKeyMap.get(chatOpenKey)!
    }

    document.querySelector('.modalContainer')!.classList.remove('enabled')
})

document.querySelectorAll<HTMLInputElement>('.langmodeRadio').forEach(element => {
    element.addEventListener('change', () => {
        localStorage.setItem('HangulSwitchMode', (+(element.id == 'lang_ctrl')).toString())
        ipcRenderer.sendSync('Patcher::setLangMode', +(element.id == 'lang_ctrl'))
    })
})

document.querySelector<HTMLInputElement>('#ingameHangulBlock')?.addEventListener('change', ({ target }) => {
    const self = target as HTMLInputElement

    localStorage.setItem('InGameHangulBlock', (+self.checked).toString())
    ipcRenderer.sendSync('Patcher::setInGameHangulBlock', self.checked)
})

document.querySelector<HTMLInputElement>('#keepHangul')?.addEventListener('change', ({ target }) => {
    const self = target as HTMLInputElement

    localStorage.setItem('KeepHangul', (+self.checked).toString())

    ipcRenderer.sendSync('Patcher::setKeepHangul', self.checked)
})

document.querySelector<HTMLDivElement>('.start')!.addEventListener('click', ({ target }) => {
    const self = target as HTMLDivElement

    self.classList.toggle('pause')
    document.querySelector<HTMLDivElement>('.isActive')!.classList.toggle('activated')

    if (self.classList.contains('pause')) {
        ipcRenderer.sendSync('Patcher::startHook')
    } else {
        ipcRenderer.sendSync('Patcher::stopHook')
    }
})

document.querySelector<HTMLSpanElement>('.github')!.addEventListener('click', () => {
    Electron.shell.openExternal('https://github.com/muno9748/HangulPatcher')
})

document.querySelector<HTMLSpanElement>('.exit')!.addEventListener('click', () => {
    ipcRenderer.sendSync('Patcher::stopHook')
    Electron.remote.getCurrentWindow().close()
})

if (!localStorage.getItem('ChatOpenVKCode')) localStorage.setItem('ChatOpenVKCode', 0x54.toString())
if (!localStorage.getItem('HangulSwitchMode')) localStorage.setItem('HangulSwitchMode', '0')
if (!localStorage.getItem('InGameHangulBlock')) localStorage.setItem('InGameHangulBlock', '1')
if (!localStorage.getItem('KeepHangul')) localStorage.setItem('KeepHangul', '0')

const reverseKeyMap = new Map(Array.from(keyMap.entries()).map(([k, v]) => [v, k]))

const chatOpenVKCode = +localStorage.getItem('ChatOpenVKCode')!

document.querySelector<HTMLButtonElement>('.Key')!.innerText = reverseKeyMap.get(chatOpenVKCode)!

const hangulSwitchMode = +localStorage.getItem('HangulSwitchMode')!

document.querySelector<HTMLInputElement>('#lang_lang')!.checked = hangulSwitchMode == 0
document.querySelector<HTMLInputElement>('#lang_ctrl')!.checked = hangulSwitchMode == 1

const inGameHangulBlock = +localStorage.getItem('InGameHangulBlock')!

document.querySelector<HTMLInputElement>('#ingameHangulBlock')!.checked = inGameHangulBlock == 1

const keepHangul = +localStorage.getItem('KeepHangul')!

document.querySelector<HTMLInputElement>('#keepHangul')!.checked = keepHangul == 1








const currentVersion = '2.0.0'

document.querySelector<HTMLSpanElement>('.version')!.innerText = currentVersion


ipcRenderer.sendSync('Patcher::setLangMode', hangulSwitchMode)
ipcRenderer.sendSync('Patcher::setChatOpen', chatOpenVKCode)
ipcRenderer.sendSync('Patcher::setKeepHangul', !!keepHangul)
ipcRenderer.sendSync('Patcher::setInGameHangulBlock', !!inGameHangulBlock)
ipcRenderer.sendSync('Patcher::startHook')