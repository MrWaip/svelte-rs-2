App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div></div>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let style = $.prop($$props, "style", 3, ""), size = $.prop($$props, "size", 3, "s"), label = $.prop($$props, "label", 3, "");
	var $$exports = { ...$.legacy_api() };
	var div = root();
	let styles;
	$.template_effect(() => {
		styles = $.set_style(div, style(), styles, { width: "100px" });
		$.set_class(div, 1, `box ${size() ?? ""}`);
		$.set_attribute(div, "data-label", label());
	});
	$.append($$anchor, div);
	return $.pop($$exports);
}
