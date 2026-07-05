App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let name = $.prop($$props, "name", 7);
	var $$exports = {
		...$.legacy_api(),
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
customElements.define("my-element", $.create_custom_element(App, { name: {} }, [], [], { mode: "open" }));
