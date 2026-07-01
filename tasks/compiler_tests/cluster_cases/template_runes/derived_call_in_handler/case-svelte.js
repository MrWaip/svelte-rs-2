import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>go</button>`);
export default function App($$anchor) {
	const noop = () => {};
	var button = root();
	$.delegated("click", button, () => {
		let count = 0;
		let double = $.derived(() => count * 2);
		noop($.get(double));
	});
	$.append($$anchor, button);
}
$.delegate(["click"]);
