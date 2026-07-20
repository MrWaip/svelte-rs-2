import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div>hi</div>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	let a = $.prop($$props, "a", 12, 1);
	a();
	var $$exports = {
		get a() {
			return a();
		},
		set a($$value) {
			a($$value);
			$.flush();
		}
	};
	var div = root();
	$.append($$anchor, div);
	return $.pop($$exports);
}
customElements.define("x-extra", $.create_custom_element(App, { a: {} }, [], [], { mode: "open" }));
