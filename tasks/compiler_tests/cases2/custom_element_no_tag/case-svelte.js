import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let name = $.prop($$props, "name", 7);
	var $$exports = {
		get name() {
			return name();
		},
		set name($$value) {
			name($$value);
			$.flush();
		}
	};
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, name()));
	$.append($$anchor, p);
	return $.pop($$exports);
}
$.create_custom_element(App, { name: {} }, [], [], { mode: "open" });
