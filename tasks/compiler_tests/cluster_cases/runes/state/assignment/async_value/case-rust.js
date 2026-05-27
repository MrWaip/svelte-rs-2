import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let a = $.state(0);
	let b = $.state(0);
	async function update() {
		await (async ($$value) => {
			var $$array = $.to_array($$value, 2);
			$.set(a, $$array[0], true);
			$.set(b, $$array[1], true);
		})([1, await Promise.resolve(2)]);
	}
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${$.get(a) ?? ""}${$.get(b) ?? ""}`));
	$.delegated("click", button, update);
	$.append($$anchor, button);
}
$.delegate(["click"]);
