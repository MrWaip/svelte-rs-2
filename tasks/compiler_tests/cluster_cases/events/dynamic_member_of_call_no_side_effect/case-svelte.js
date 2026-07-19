import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>go</button>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let make = (name) => ({ handler: () => console.log(name) });
	var button = root();
	var event_handler = $.derived(() => make("Tama").handler);
	$.delegated("click", button, function(...$$args) {
		$.get(event_handler)?.apply(this, $$args);
	});
	$.append($$anchor, button);
	$.pop();
}
$.delegate(["click"]);
