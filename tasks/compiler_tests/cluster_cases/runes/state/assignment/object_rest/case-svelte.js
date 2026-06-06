import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let a = $.state(0);
	let rest = $.state(0);
	let obj = {
		a: 1,
		x: 2
	};
	function update() {
		$.set(a, obj.a, true), $.set(rest, $.exclude_from_object(obj, ["a"]), true);
	}
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$.get(a) ?? ""}${$.get(rest) ?? ""}`));
	$.delegated("click", button, update);
	$.append($$anchor, button);
}
$.delegate(["click"]);
