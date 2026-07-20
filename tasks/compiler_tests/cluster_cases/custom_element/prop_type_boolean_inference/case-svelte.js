import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div>hi</div>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	let red = $.prop($$props, "red", 12, false);
	red();
	var $$exports = {
		get red() {
			return red();
		},
		set red($$value) {
			red($$value);
			$.flush();
		}
	};
	var div = root();
	$.append($$anchor, div);
	return $.pop($$exports);
}
customElements.define("x-bool", $.create_custom_element(App, { red: {
	reflect: true,
	type: "Boolean"
} }, [], [], { mode: "open" }));
