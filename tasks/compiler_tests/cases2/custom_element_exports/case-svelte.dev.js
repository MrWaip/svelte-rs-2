App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let value = $.prop($$props, "value", 7, 0);
	function reset() {
		value(0);
	}
	var $$exports = {
		...$.legacy_api(),
		get reset() {
			return reset;
		},
		get value() {
			return value();
		},
		set value($$value = 0) {
			value($$value);
			$.flush();
		}
	};
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, value()));
	$.append($$anchor, p);
	return $.pop($$exports);
}
customElements.define("my-widget", $.create_custom_element(App, { value: {} }, [], ["reset"], { mode: "open" }));
