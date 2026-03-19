import * as $ from "svelte/internal/client";
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let value = $.prop($$props, "value", 7, 0);
	function reset() {
		value(0);
	}
	var $$exports = {
		reset,
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
