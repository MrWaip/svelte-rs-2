import * as $ from "svelte/internal/client";
var root = $.from_html(`<button></button>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let test = $.prop($$props, "test", 7);
	var button = root();
	$.delegated("click", button, () => {
		//svelte-ignore ownership_invalid_mutation
		test().test = Math.random();
	});
	$.append($$anchor, button);
	$.pop();
}
$.delegate(["click"]);
