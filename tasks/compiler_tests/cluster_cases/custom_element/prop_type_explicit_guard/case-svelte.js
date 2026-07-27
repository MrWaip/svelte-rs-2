import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div>hi</div>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	let count = $.prop($$props, "count", 12, 0);
	count();
	var $$exports = {
		get count() {
			return count();
		},
		set count($$value) {
			count($$value);
			$.flush();
		}
	};
	var div = root();
	$.append($$anchor, div);
	return $.pop($$exports);
}
customElements.define("x-num", $.create_custom_element(App, { count: { type: "Number" } }, [], [], { mode: "open" }));
