App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let count = $.prop($$props, "count", 7), label = $.prop($$props, "label", 7);
	var $$exports = {
		...$.legacy_api(),
		get count() {
			return count();
		},
		set count($$value) {
			count($$value);
			$.flush();
		},
		get label() {
			return label();
		},
		set label($$value) {
			label($$value);
			$.flush();
		}
	};
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${label() ?? ""}: ${count() ?? ""}`));
	$.append($$anchor, p);
	return $.pop($$exports);
}
customElements.define("my-counter", $.create_custom_element(App, {
	count: {
		reflect: true,
		type: "Number"
	},
	label: { attribute: "data-label" }
}, [], [], { mode: "open" }));
