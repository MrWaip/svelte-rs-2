import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div>hi</div>`), App[$.FILENAME], [[8, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let n = $.prop($$props, "n", 12, 0);
	n();
	var $$exports = {
		...$.legacy_api(),
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
