import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div>hi</div>`), App[$.FILENAME], [[8, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let count = $.prop($$props, "count", 12, 0);
	count();
	var $$exports = {
		...$.legacy_api(),
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
