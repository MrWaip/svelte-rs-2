import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor, $$props) {
	let a = $.prop($$props, "a", 7, 0), b = $.prop($$props, "b", 7, 0);
	function f() {
		(($$value) => {
			var $$array = $.to_array($$value, 2);
			a($$array[0]);
			b($$array[1]);
		})([1, 2]);
	}
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${a() ?? ""}${b() ?? ""}`));
	$.delegated("click", button, f);
	$.append($$anchor, button);
}
$.delegate(["click"]);
