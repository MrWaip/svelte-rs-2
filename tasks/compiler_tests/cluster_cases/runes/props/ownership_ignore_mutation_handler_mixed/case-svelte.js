import * as $ from "svelte/internal/client";
var root = $.from_html(`<button></button>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let a = $.prop($$props, "a", 7), b = $.prop($$props, "b", 7);
	var button = root();
	$.delegated("click", button, () => {
		//svelte-ignore ownership_invalid_mutation
		a().x = 1;
		b().x = 2;
	});
	$.append($$anchor, button);
	$.pop();
}
$.delegate(["click"]);
