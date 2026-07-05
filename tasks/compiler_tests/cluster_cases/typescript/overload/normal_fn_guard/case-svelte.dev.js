App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	function f(a) {
		return a + 1;
	}
	let count = $.tag($.state(0), "count");
	const r = f(1);
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${r ?? ""}${$.get(count) ?? ""}`));
	$.delegated("click", button, function click() {
		return $.update(count);
	});
	$.append($$anchor, button);
	return $.pop($$exports);
}
$.delegate(["click"]);
