import * as $ from "svelte/internal/client";
var rest_excludes = new Set([
	"$$slots",
	"$$events",
	"$$legacy"
]);
var root = $.from_html(`<p> </p>`);
export default function App($$anchor, $$props) {
	const props = $.rest_props($$props, rest_excludes);
	var p = root();
	var text = $.child(p, true);
	$.reset(p);
	$.template_effect(($0) => $.set_text(text, $0), [() => Object.keys(props)]);
	$.append($$anchor, p);
}
