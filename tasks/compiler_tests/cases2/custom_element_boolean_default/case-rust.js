import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let active = $.prop($$props, "active", 7, false);
	var $$exports = {
		get active() {
			return active();
		},
		set active($$value = false) {
			active($$value);
			$.flush();
		}
	};
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, active()));
	$.append($$anchor, p);
	return $.pop($$exports);
}
customElements.define("my-toggle", $.create_custom_element(App, { active: {} }, [], [], { mode: "open" }));
