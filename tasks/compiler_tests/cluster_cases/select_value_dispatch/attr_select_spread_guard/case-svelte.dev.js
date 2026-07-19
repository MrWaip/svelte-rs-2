App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<select><option>Dog</option></select>`), App[$.FILENAME], [[
	5,
	0,
	[[6, 1]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var select = root();
	$.attribute_effect(select, () => ({
		value: "dog",
		...$$props.props
	}));
	var option = $.child(select);
	option.value = option.__value = "dog";
	$.reset(select);
	$.append($$anchor, select);
	return $.pop($$exports);
}
