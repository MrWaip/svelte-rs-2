App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<button> </button>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let a = $.prop($$props, "a", 7, 0), b = $.prop($$props, "b", 7, 0);
	function f() {
		(($$value) => {
			a($$value.a);
			b($$value.b);
		})({
			a: 1,
			b: 2
		});
	}
	var $$exports = { ...$.legacy_api() };
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${a() ?? ""}${b() ?? ""}`));
	$.delegated("click", button, f);
	$.append($$anchor, button);
	return $.pop($$exports);
}
$.delegate(["click"]);
