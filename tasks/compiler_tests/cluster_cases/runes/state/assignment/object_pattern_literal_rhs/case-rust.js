import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let a = $.state(0);
	let b = $.state(0);
	function update() {
		(($$value) => {
			$.set(a, $$value.a, true);
			$.set(b, $$value.b, true);
		})({
			a: 1,
			b: 2
		});
	}
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$.get(a) ?? ""}${$.get(b) ?? ""}`));
	$.delegated("click", button, update);
	$.append($$anchor, button);
}
$.delegate(["click"]);
