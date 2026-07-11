import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[9, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let a = $.tag($.mutable_source(), "a");
	let b = $.tag($.mutable_source(), "b");
	const obj = {
		a: 1,
		b: 2
	};
	function run() {
		$.set(a, obj.a), $.set(b, obj.b);
	}
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$.get(a) ?? ""}${$.get(b) ?? ""}`));
	$.event("click", button, run);
	$.append($$anchor, button);
	return $.pop($$exports);
}
