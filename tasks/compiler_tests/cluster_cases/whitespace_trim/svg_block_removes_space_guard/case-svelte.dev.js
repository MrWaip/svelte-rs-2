App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_svg(`<rect></rect><rect></rect>`, 1), App[$.FILENAME], [[7, 2], [8, 2]]);
var root_1 = $.add_locations($.from_svg(`<svg></svg>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var svg = root_1();
	$.add_svelte_meta(() => $.each(svg, 21, () => $$props.items, $.index, ($$anchor, i) => {
		var fragment = root();
		$.next();
		$.append($$anchor, fragment);
	}), "each", App, 6, 1);
	$.reset(svg);
	$.append($$anchor, svg);
	return $.pop($$exports);
}
