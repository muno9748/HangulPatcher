import { app, BrowserWindow, ipcMain } from 'electron'
import * as Patcher from './Patcher.node'
import * as path from 'path'

const isDev = process.env.HP_DEV == '1'
let mainWindow: BrowserWindow

function createWindow() {
    mainWindow = new BrowserWindow({
        width: 900,
        height: 500,
        center: true,
        kiosk: !isDev,
        resizable: false,
        fullscreen: false,
        fullscreenable: false,
        webPreferences: {
            nodeIntegration: true,
            contextIsolation: false,
            enableRemoteModule: true
        },
        frame: false,
    })

    if (isDev) {
        mainWindow.loadFile('./dist/source/index.html')
    } else {
        mainWindow.loadURL(path.join(process.cwd(), 'resources', 'app', 'source', 'index.html'))
    }
    mainWindow.setMenu(null)

    mainWindow.on('closed', () => (mainWindow = undefined!))
    mainWindow.focus()

    ipcMain.on('openMain', () => {
        const oldWindow = mainWindow

        mainWindow = new BrowserWindow({
            width: 900,
            height: 500,
            center: true,
            kiosk: !isDev,
            resizable: false,
            fullscreen: false,
            fullscreenable: false,
            webPreferences: {
                nodeIntegration: true,
                contextIsolation: false,
                enableRemoteModule: true
            },
            frame: false
        })

        mainWindow.loadURL(isDev ? 'http://localhost:3000/app.html' : path.join(process.cwd(), 'resources', 'app', 'source', 'app.html'))
        mainWindow.setMenu(null)

        mainWindow.on('closed', () => (mainWindow = undefined!))
        mainWindow.focus()

        oldWindow.close()
    })

    ipcMain.on('Patcher::startHook', (e) => {
        Patcher.startHook()
        e.returnValue = ""
    })

    ipcMain.on('Patcher::stopHook', (e) => {
        Patcher.stopHook()
        e.returnValue = ""
    })

    ipcMain.on('Patcher::setLangMode', (e, mode) => {
        Patcher.setLangmode(mode)
        e.returnValue = ""
    })

    ipcMain.on('Patcher::setChatOpen', (e, vkCode) => {
        Patcher.setChatOpen(vkCode)
        e.returnValue = ""
    })

    ipcMain.on('Patcher::setCmdOpen', (e, vkCode) => {
        Patcher.setCmdOpen(vkCode)
        e.returnValue = ""
    })

    ipcMain.on('Patcher::setKeepHangul', (e, value) => {
        Patcher.setKeepHangul(value)
        e.returnValue = ""
    })

    ipcMain.on('Patcher::setInGameHangulBlock', (e, value) => {
        Patcher.setInGameHangulBlock(value)
        e.returnValue = ""
    })
}

app.on('ready', createWindow)

app.on('window-all-closed', () => {
    if (process.platform != 'darwin') app.quit()
})

app.on('activate', () => {
    if (mainWindow == null) createWindow()
})