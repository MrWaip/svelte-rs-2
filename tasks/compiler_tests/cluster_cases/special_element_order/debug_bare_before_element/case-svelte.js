import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>+</button>`);
export default function App($$anchor) {
	let count = $.state(0);
	var button = root();
	$.template_effect(() => {
		console.log({});
		debugger;
	});
	$.delegated("click", button, () => $.update(count));
	$.append($$anchor, button);
}
$.delegate(["click"]);
