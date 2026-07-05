import * as $ from "svelte/internal/client";
var rest_excludes = new Set([
	"$$slots",
	"$$events",
	"$$legacy"
]);
var root = $.from_html(`<p>spread</p>`);
export default function App($$anchor, $$props) {
	let rest = $.rest_props($$props, rest_excludes);
	var p = root();
	$.attribute_effect(p, () => ({
		...rest,
		"data-extra": "x"
	}), void 0, void 0, void 0, "svelte-qv4ee3");
	$.append($$anchor, p);
}
