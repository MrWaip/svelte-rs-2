import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div>hi</div>`), App[$.FILENAME], [[8, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let red = $.prop($$props, "red", 12, false);
	red();
	var $$exports = {
		...$.legacy_api(),
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
