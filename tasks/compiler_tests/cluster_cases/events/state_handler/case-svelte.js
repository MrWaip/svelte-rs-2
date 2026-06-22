import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>x</button>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let handler = $.state(() => {});
	$.user_effect(() => {
		$.set(handler, () => console.log("x"));
	});
	var button = root();
	$.delegated("click", button, function(...$$args) {
		$.get(handler)?.apply(this, $$args);
	});
	$.append($$anchor, button);
	$.pop();
}
$.delegate(["click"]);
