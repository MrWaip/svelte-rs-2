import * as $ from "svelte/internal/client";
var root = $.from_html(`<p>spread</p>`);
export default function App($$anchor, $$props) {
	let rest = $.rest_props($$props, [
		"$$slots",
		"$$events",
		"$$legacy"
	]);
	var p = root();
	$.attribute_effect(p, () => ({
		...rest,
		"data-extra": "x"
	}), void 0, void 0, void 0, "svelte-qv4ee3");
	$.append($$anchor, p);
}
