import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<div>hi</div>`);
export default function App($$anchor, $$props) {
	$.push($$props, false);
	let foo = $.prop($$props, "foo", 12);
	foo();
	var $$exports = {
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
