import * as $ from "svelte/internal/client";
var root_1 = $.from_svg(`<a></a>`);
var root = $.from_svg(`<svg></svg>`);
export default function App($$anchor) {
	let items = $.proxy([1]);
	var svg = root();
	$.each(svg, 21, () => items, $.index, ($$anchor, item) => {
		var a = root_1();
		$.template_effect(() => $.set_attribute(a, "href", `/node-${$.get(item) ?? ""}`));
		$.append($$anchor, a);
	});
	$.reset(svg);
	$.append($$anchor, svg);
}
