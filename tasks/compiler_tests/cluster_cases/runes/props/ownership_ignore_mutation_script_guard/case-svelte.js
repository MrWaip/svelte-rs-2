import * as $ from "svelte/internal/client";
var root = $.from_html(`<button></button>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let test = $.prop($$props, "test", 7);
	function go() {
		//svelte-ignore ownership_invalid_mutation
		test().test = Math.random();
	}
	var button = root();
	$.delegated("click", button, go);
	$.append($$anchor, button);
	$.pop();
}
$.delegate(["click"]);
