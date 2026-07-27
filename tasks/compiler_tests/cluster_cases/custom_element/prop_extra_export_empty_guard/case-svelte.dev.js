import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div>hi</div>`), App[$.FILENAME], [[8, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let a = $.prop($$props, "a", 12, 1);
	a();
	var $$exports = {
		...$.legacy_api(),
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
