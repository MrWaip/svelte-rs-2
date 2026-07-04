App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<section><span><span></span></span> <div><div><div>text</div></div></div> <p><b><i></i></b></p></section>`), App[$.FILENAME], [[
	2,
	0,
	[
		[
			3,
			2,
			[[4, 4]]
		],
		[
			7,
			2,
			[[
				8,
				4,
				[[9, 6]]
			]]
		],
		[
			13,
			2,
			[[
				14,
				4,
				[[15, 6]]
			]]
		]
	]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var section = root();
	var span = $.child(section);
	var span_1 = $.child(span);
	span_1.textContent = name;
	$.reset(span);
	var p = $.sibling(span, 4);
	var b = $.child(p);
	var i = $.child(b);
	$.set_attribute(i, "name", name);
	$.reset(b);
	$.reset(p);
	$.reset(section);
	$.append($$anchor, section);
	return $.pop($$exports);
}
