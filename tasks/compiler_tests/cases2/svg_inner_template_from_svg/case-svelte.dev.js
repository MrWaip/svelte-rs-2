App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_svg(`<circle></circle>`), App[$.FILENAME], [[7, 2]]);
var root_1 = $.add_locations($.from_svg(`<svg></svg>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let items = $.tag_proxy($.proxy([
		1,
		2,
		3
	]), "items");
	var $$exports = { ...$.legacy_api() };
	var svg = root_1();
	$.add_svelte_meta(() => $.each(svg, 21, () => items, $.index, ($$anchor, item) => {
		var circle = root();
		$.set_attribute(circle, "cy", 10);
		$.set_attribute(circle, "r", 5);
		$.template_effect(() => $.set_attribute(circle, "cx", $.get(item) * 10));
		$.append($$anchor, circle);
	}), "each", App, 6, 1);
	$.reset(svg);
	$.append($$anchor, svg);
	return $.pop($$exports);
}
