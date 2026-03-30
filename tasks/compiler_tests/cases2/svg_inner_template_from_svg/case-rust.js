import * as $ from "svelte/internal/client";
var root_1 = $.from_svg(`<circle></circle>`);
var root = $.from_svg(`<svg></svg>`);
export default function App($$anchor) {
	let items = $.proxy([
		1,
		2,
		3
	]);
	var svg = root();
	$.each(svg, 21, () => items, $.index, ($$anchor, item) => {
		var circle = root_1();
		$.set_attribute(circle, "cy", 10);
		$.set_attribute(circle, "r", 5);
		$.template_effect(() => $.set_attribute(circle, "cx", $.get(item) * 10));
		$.append($$anchor, circle);
	});
	$.reset(svg);
	$.append($$anchor, svg);
}
