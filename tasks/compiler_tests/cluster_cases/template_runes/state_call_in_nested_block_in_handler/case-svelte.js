import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>go</button>`);
export default function App($$anchor) {
	const noop = () => {};
	var button = root();
	$.delegated("click", button, () => {
		if (noop) {
			const s = $.proxy({ x: 1 });
			noop(s);
		}
	});
	$.append($$anchor, button);
}
$.delegate(["click"]);
