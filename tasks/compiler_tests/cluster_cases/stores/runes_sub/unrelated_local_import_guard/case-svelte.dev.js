App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { helper } from "./store.js";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let a = $.tag($.state(0), "a");
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$.get(a) ?? ""} ${helper ?? ""}`));
	$.delegated("click", button, function click() {
		return $.update(a);
	});
	$.append($$anchor, button);
	return $.pop($$exports);
}
$.delegate(["click"]);
