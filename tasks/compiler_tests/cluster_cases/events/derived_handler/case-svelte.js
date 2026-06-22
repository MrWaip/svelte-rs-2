import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>x</button>`);
export default function App($$anchor) {
	let flag = true;
	const handler_1 = () => {};
	const handler_2 = () => {};
	let handler = $.derived(() => flag ? handler_1 : handler_2);
	var button = root();
	$.delegated("click", button, function(...$$args) {
		$.get(handler)?.apply(this, $$args);
	});
	$.append($$anchor, button);
}
$.delegate(["click"]);
