import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let total = $.prop($$props, "count", 7, 0);
	var $$exports = {
		get count() {
			return total();
		},
		set count($$value = 0) {
			total($$value);
			$.flush();
		}
	};
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, total()));
	$.append($$anchor, p);
	return $.pop($$exports);
}
customElements.define("my-counter", $.create_custom_element(App, { count: {} }, [], [], { mode: "open" }));
