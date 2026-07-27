App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var optgroup_content = $.add_locations($.with_script($.from_html(`<option>A</option> <script>console.log('hi')<\/script>`, 1)), App[$.FILENAME], [[3, 2], [4, 2]]);
var root = $.add_locations($.from_html(`<select><optgroup label="g"><!></optgroup></select>`), App[$.FILENAME], [[
	1,
	0,
	[[2, 1]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var select = root();
	var optgroup = $.child(select);
	$.customizable_select(optgroup, () => {
		var anchor = $.child(optgroup);
		var fragment = optgroup_content();
		var option = $.first_child(fragment);
		option.value = option.__value = "a";
		$.next(2);
		$.append(anchor, fragment);
	});
	$.reset(select);
	$.append($$anchor, select);
	return $.pop($$exports);
}
