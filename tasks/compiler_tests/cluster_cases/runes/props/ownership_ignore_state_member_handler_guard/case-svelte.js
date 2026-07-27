import * as $ from "svelte/internal/client";
var root = $.from_html(`<button></button>`);
export default function App($$anchor) {
	let s = $.proxy({ x: 0 });
	var button = root();
	$.delegated("click", button, () => {
		//svelte-ignore ownership_invalid_mutation
		s.x = 1;
	});
	$.append($$anchor, button);
}
$.delegate(["click"]);
