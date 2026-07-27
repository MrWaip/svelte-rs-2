import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div>hi</div>`), App[$.FILENAME], [[8, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let foo = $.prop($$props, "foo", 12);
	foo();
	var $$exports = {
		...$.legacy_api(),
		get foo() {
			return foo();
		},
		set foo($$value) {
			foo($$value);
			$.flush();
		}
	};
	var div = root();
	$.append($$anchor, div);
	return $.pop($$exports);
}
customElements.define("x-attr", $.create_custom_element(App, { foo: { attribute: "foo-bar" } }, [], [], { mode: "open" }));
