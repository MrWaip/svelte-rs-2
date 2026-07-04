import * as $ from "svelte/internal/client";
var rest_excludes = new Set([
	"$$slots",
	"$$events",
	"$$legacy",
	"$$host",
	"x"
]);
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	$.push($$props, true);
	let x = $.prop($$props, "x", 7, 0), rest = $.rest_props($$props, rest_excludes);
	let rawData = $.state({
		a: 1,
		b: 2
	});
	let snapshot = $.snapshot($.get(rawData));
	var $$exports = {
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
