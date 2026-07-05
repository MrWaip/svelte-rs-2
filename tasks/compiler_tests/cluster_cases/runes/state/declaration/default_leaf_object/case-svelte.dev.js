App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[4, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let tmp = {}, a = $.tag_proxy($.proxy($.fallback(tmp.a, 10)), "a"), b = $.tag_proxy($.proxy($.fallback(tmp.b, 20)), "b");
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${a ?? ""}${b ?? ""}`));
	$.append($$anchor, button);
	return $.pop($$exports);
}
