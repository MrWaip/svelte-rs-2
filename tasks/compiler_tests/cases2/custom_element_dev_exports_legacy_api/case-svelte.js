import * as $ from "svelte/internal/client";
var rest_excludes = new Set([
	"$$slots",
	"$$events",
	"$$legacy",
	"$$host",
	"x"
]);
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let x = $.prop($$props, "x", 7, 0), rest = $.rest_props($$props, rest_excludes);
	const VERSION = "1";
	function helper() {}
	var $$exports = {
		VERSION,
		helper,
		get x() {
			return x();
		},
		set x($$value = 0) {
			x($$value);
			$.flush();
		}
	};
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, x()));
	$.append($$anchor, p);
	return $.pop($$exports);
}
customElements.define("my-el", $.create_custom_element(App, { x: {} }, [], ["VERSION", "helper"], { mode: "open" }));
