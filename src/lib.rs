use neon::prelude::*;
use fs_mod_parser::{ModParserOptions, parse_mod_with_options, parse_savegame};

fn parse_mod_first_pass(mut cx: FunctionContext) -> JsResult<JsString> {
    let options = ModParserOptions{
        include_mod_detail: true,
        skip_detail_icons : true,
        ..Default::default()
    };

    if let Ok(file_name) = cx.argument::<JsString>(0) {
        let file_name = file_name.value(&mut cx);
        let result_json = parse_mod_with_options(file_name, &options).to_json();
        return Ok(cx.string(result_json));
    }
    Ok(cx.string("{}"))
}

fn parse_mod_last_pass(mut cx: FunctionContext) -> JsResult<JsString> {
    let options = ModParserOptions{
        include_mod_detail: true,
        skip_detail_icons : false,
        ..Default::default()
    };

    if let Ok(file_name) = cx.argument::<JsString>(0) {
        let file_name = file_name.value(&mut cx);
        let result_json = parse_mod_with_options(file_name, &options).to_json();
        return Ok(cx.string(result_json));
    }
    Ok(cx.string("{}"))
}

fn parse_save_game(mut cx: FunctionContext) -> JsResult<JsString> {
    if let Ok(file_name) = cx.argument::<JsString>(0) {
        let file_name = file_name.value(&mut cx);
        let result_json = parse_savegame(file_name).to_json();
        return Ok(cx.string(result_json));
    }
    Ok(cx.string("{}"))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("parseModFirstPass", parse_mod_first_pass)?;
    cx.export_function("parseModLastPass", parse_mod_last_pass)?;
    cx.export_function("parseSaveGame", parse_save_game)?;
    Ok(())
}
