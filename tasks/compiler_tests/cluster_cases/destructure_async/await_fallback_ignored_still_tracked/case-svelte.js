import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	let f = $.state(0);
	async function go() {
		// svelte-ignore await_reactivity_loss
		await (async ($$value) => {
			var $$array = $.to_array($$value, 1);
			$.set(f, await $.fallback($$array[0], async () => false || await Promise.resolve(6), true), true);
		})([]);
	}
	var button = root();
	var text = $.child(button, true);
	$.reset(button);
	$.template_effect(() => $.set_text(text, $.get(f)));
	$.delegated("click", button, go);
	$.append($$anchor, button);
}
$.delegate(["click"]);
