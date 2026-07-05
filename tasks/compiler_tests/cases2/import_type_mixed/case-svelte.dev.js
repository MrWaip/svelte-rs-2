App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import { realValue } from "./utils";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[12, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let data = $.tag_proxy($.proxy({ value: 0 }), "data");
	function process(input) {
		return realValue.transform(input);
	}
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, realValue.label));
	$.delegated("click", button, function click() {
		return process(data);
	});
	$.append($$anchor, button);
	return $.pop($$exports);
}
$.delegate(["click"]);
