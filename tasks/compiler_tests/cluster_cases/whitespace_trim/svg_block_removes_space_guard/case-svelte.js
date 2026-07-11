import * as $ from "svelte/internal/client";
var root = $.from_svg(`<rect></rect><rect></rect>`, 1);
var root_1 = $.from_svg(`<svg></svg>`);
export default function App($$anchor, $$props) {
	var svg = root_1();
	$.each(svg, 21, () => $$props.items, $.index, ($$anchor, i) => {
		var fragment = root();
		$.next();
		$.append($$anchor, fragment);
	});
	$.reset(svg);
	$.append($$anchor, svg);
}
