import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>x</button>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	let store = $.prop($$props, "store", 8, undefined);
	$.init();
	var button = root();
	$.event("click", button, function(...$$args) {
		(store()?.reset)?.apply(this, $$args);
	});
	$.append($$anchor, button);
	$.pop();
}
