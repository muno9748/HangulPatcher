#![allow(non_upper_case_globals)]

extern crate sejong;
extern crate winapi;

mod api;
mod buf;
mod patcher;
mod send_key;

use neon::prelude::*;

fn start_hook(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    unsafe {
        patcher::start_hook();
    }

    Ok(cx.undefined())
}

fn stop_hook(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    unsafe {
        patcher::stop_hook();
    }

    Ok(cx.undefined())
}

fn wait(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    unsafe {
        patcher::wait();
    }

    Ok(cx.undefined())
}

fn set_langmode(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let value = cx.argument::<JsNumber>(0)?.value(&mut cx);

    if value != 0. && value != 1. {
        panic!("Unexpected value");
    }

    unsafe {
        patcher::set_langmode(value as u8);
    }

    Ok(cx.undefined())
}

fn set_chat_open(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let value = cx.argument::<JsNumber>(0)?.value(&mut cx);

    unsafe {
        patcher::set_chat_open(value as u32);
    }

    Ok(cx.undefined())
}

fn set_hangul_block(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let value = cx.argument::<JsBoolean>(0)?.value(&mut cx);

    unsafe {
        patcher::set_hangul_block(value);
    }

    Ok(cx.undefined())
}

fn set_keep_hangul(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let value = cx.argument::<JsBoolean>(0)?.value(&mut cx);

    unsafe {
        patcher::set_keep_hangul(value);
    }

    Ok(cx.undefined())
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("startHook", start_hook)?;
    cx.export_function("stopHook", stop_hook)?;
    cx.export_function("setLangmode", set_langmode)?;
    cx.export_function("setChatOpen", set_chat_open)?;
    cx.export_function("setKeepHangul", set_keep_hangul)?;
    cx.export_function("setInGameHangulBlock", set_hangul_block)?;
    cx.export_function("wait", wait)?;

    Ok(())
}
