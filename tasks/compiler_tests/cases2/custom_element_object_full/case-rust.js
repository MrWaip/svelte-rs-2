import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let count = $.prop($$props, "count", 7, 0);
	var $$exports = {
		get count() {
			return count();
		},
		set count($$value = 0) {
			count($$value);
			$.flush();
		}
	};
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, count()));
	$.append($$anchor, p);
	return $.pop($$exports);
}
customElements.define("my-component", $.create_custom_element(App, { count: {
	reflect: true,
	type: "Number"
} }, [], [], { mode: "open" }));
