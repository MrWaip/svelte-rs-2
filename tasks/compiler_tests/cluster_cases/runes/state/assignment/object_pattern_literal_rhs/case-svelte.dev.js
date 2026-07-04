App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[8, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let a = $.tag($.state(0), "a");
	let b = $.tag($.state(0), "b");
	function update() {
		(($$value) => {
			$.set(a, $$value.a, true);
			$.set(b, $$value.b, true);
		})({
			a: 1,
			b: 2
		});
	}
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$.get(a) ?? ""}${$.get(b) ?? ""}`));
	$.delegated("click", button, update);
	$.append($$anchor, button);
	return $.pop($$exports);
}
$.delegate(["click"]);
