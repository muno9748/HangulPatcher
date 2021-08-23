type u8 = number
type u32 = number

declare module '*.node' {
    export function startHook(): void
    export function stopHook(): void
    export function setLangmode(value: u8): void
    export function setChatOpen(value: u32): void
    export function setCmdOpen(value: u32): void
    export function setKeepHangul(value: boolean): void
    export function setInGameHangulBlock(value: boolean): void
    export function wait(): void
}