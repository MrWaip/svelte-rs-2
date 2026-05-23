import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<button>x</button>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	let entries = $.prop($$props, "entries", 12);
	function put(item, value) {
		entries(entries()[item.id] = value, true);
	}
	$.init();
	var button = root();
	$.event("click", button, () => put({ id: "a" }, 1));
	$.append($$anchor, button);
	$.pop();
}
