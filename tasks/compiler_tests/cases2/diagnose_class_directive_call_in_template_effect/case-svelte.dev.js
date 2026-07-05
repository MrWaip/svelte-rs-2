App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div> </div>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var div = root();
	let classes;
	var text = $.child(div, true);
	$.reset(div);
	$.template_effect(($0) => {
		classes = $.set_class(div, 1, "", null, classes, $0);
		$.set_text(text, $$props.name);
	}, [() => ({
		x: $$props.a,
		y: Boolean($$props.onClick)
	})]);
	$.append($$anchor, div);
	return $.pop($$exports);
}
