App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[7, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let x = $.prop($$props, "x", 7, 0), rest = $.rest_props($$props, [
		"$$slots",
		"$$events",
		"$$legacy",
		"$$host",
		"x"
	], "rest");
	const VERSION = "1";
	function helper() {}
	var $$exports = {
		get VERSION() {
			return VERSION;
		},
		get helper() {
			return helper;
		},
		get x() {
			return x();
		},
		set x($$value = 0) {
			x($$value);
			$.flush();
		},
		...$.legacy_api()
	};
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(() => $.set_text(text, x()));
	$.append($$anchor, p);
	return $.pop($$exports);
}
customElements.define("my-el", $.create_custom_element(App, { x: {} }, [], ["VERSION", "helper"], { mode: "open" }));
