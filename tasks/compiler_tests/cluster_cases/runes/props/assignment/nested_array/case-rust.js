import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor, $$props) {
	let a = $.prop($$props, "a", 7, 0), b = $.prop($$props, "b", 7, 0), c = $.prop($$props, "c", 7, 0), d = $.prop($$props, "d", 7, 0);
	function f() {
		(($$value) => {
			var $$array = $.to_array($$value, 2);
			var $$array_1 = $.to_array($$array[0], 2);
			var $$array_2 = $.to_array($$array[1], 2);
			a($$array_1[0]);
			b($$array_1[1]);
			c($$array_2[0]);
			d($$array_2[1]);
		})([[1, 2], [3, 4]]);
	}
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${a() ?? ""}${b() ?? ""}${c() ?? ""}${d() ?? ""}`));
	$.delegated("click", button, f);
	$.append($$anchor, button);
}
$.delegate(["click"]);
