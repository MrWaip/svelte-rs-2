App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let active = $.prop($$props, "active", 7, false);
	var $$exports = {
		...$.legacy_api(),
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
