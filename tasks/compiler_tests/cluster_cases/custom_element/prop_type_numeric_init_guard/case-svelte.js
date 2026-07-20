import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div>hi</div>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	let n = $.prop($$props, "n", 12, 0);
	n();
	var $$exports = {
		get n() {
			return n();
		},
		set n($$value) {
			n($$value);
			$.flush();
		}
	};
	var div = root();
	$.append($$anchor, div);
	return $.pop($$exports);
}
customElements.define("x-numlit", $.create_custom_element(App, { n: { reflect: true } }, [], [], { mode: "open" }));
