import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>go</button>`);
export default function App($$anchor) {
	const noop = () => {};
	var button = root();
	$.delegated("click", button, () => {
		const n = 5;
		noop(n);
	});
	$.append($$anchor, button);
}
$.delegate(["click"]);
