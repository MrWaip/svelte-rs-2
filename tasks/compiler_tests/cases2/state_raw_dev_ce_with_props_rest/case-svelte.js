App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var rest_excludes = new Set([
	"$$slots",
	"$$events",
	"$$legacy",
	"$$host",
	"x"
]);
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[7, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let x = $.prop($$props, "x", 7, 0), rest = $.rest_props($$props, rest_excludes, "rest");
	let rawData = $.tag($.state({
		a: 1,
		b: 2
	}), "rawData");
	let snapshot = $.snapshot($.get(rawData));
	var $$exports = {
		...$.legacy_api(),
		get x() {
			return x();
		},
		set x($$value = 0) {
			x($$value);
			$.flush();
		}
	};
	var p = root();
	var text = $.child(p);
	$.reset(p);
	$.template_effect(() => $.set_text(text, `${x() ?? ""} ${snapshot.a ?? ""}`));
	$.append($$anchor, p);
	return $.pop($$exports);
}
$.create_custom_element(App, { x: {} }, [], [], { mode: "open" });
