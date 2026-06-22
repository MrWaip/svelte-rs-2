import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>x</button>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	const handler_a = $.mutable_source();
	let flag = true;
	const handler_1 = () => {};
	const handler_2 = () => {};
	$.legacy_pre_effect(() => {}, () => {
		$.set(handler_a, flag ? handler_1 : handler_2);
	});
	$.legacy_pre_effect_reset();
	var button = root();
	$.event("click", button, function(...$$args) {
		$.get(handler_a)?.apply(this, $$args);
	});
	$.append($$anchor, button);
	$.pop();
}
