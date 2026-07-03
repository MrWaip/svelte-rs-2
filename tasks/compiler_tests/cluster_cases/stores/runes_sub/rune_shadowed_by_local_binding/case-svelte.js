import * as $ from "svelte/internal/client";
var root = $.from_html(`<button> </button>`);
export default function App($$anchor) {
	const $state = () => $.store_get(state, "$state", $$stores);
	const [$$stores, $$cleanup] = $.setup_stores();
	let state = 5;
	let foo = $state()(0);
	let bar = $.derived(() => foo + 1);
	var button = root();
	var text = $.child(button);
	$.reset(button);
	$.template_effect(() => $.set_text(text, `${foo ?? ""} ${$.get(bar) ?? ""} 5`));
	$.delegated("click", button, () => foo++);
	$.append($$anchor, button);
	$$cleanup();
}
$.delegate(["click"]);
