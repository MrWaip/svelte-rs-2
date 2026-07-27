import "svelte/internal/flags/async";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>inc</button> <div></div>`, 1);
export default function App($$anchor) {
	let x = $.state(0);
	function delay(value) {
		return Promise.resolve(value);
	}
	var fragment = root();
	var button = $.first_child(fragment);
	var div = $.sibling(button, 2);
	$.delegated("click", button, () => $.update(x));
	$.delegated("click", div, async () => await delay($.get(x)));
	$.append($$anchor, fragment);
}
$.delegate(["click"]);
