import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor, $$props) {
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
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${a() ?? ""}${b() ?? ""}`));
	$.delegated("click", button, f);
	$.append($$anchor, button);
}
$.delegate(["click"]);
