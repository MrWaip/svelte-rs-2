import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor, $$props) {
	const k = "z";
	let a = $.prop($$props, "a", 7, 0);
	function f() {
		(($$value) => {
			a($$value[k]);
		})({ z: 1 });
	}
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, a()));
	$.delegated("click", button, f);
	$.append($$anchor, button);
}
$.delegate(["click"]);
