App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var option_content = $.add_locations($.with_script($.from_html(`<b>A</b><script>console.log('hi')<\/script>`, 1)), App[$.FILENAME], [[2, 19], [2, 27]]);
var root = $.add_locations($.from_html(`<select><option><!></option></select>`), App[$.FILENAME], [[
	1,
	0,
	[[2, 1]]
]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var select = root();
	var option = $.child(select);
	$.customizable_select(option, () => {
		var anchor = $.child(option);
		var fragment = option_content();
		$.next();
		$.append(anchor, fragment);
	});
	option.value = option.__value = "a";
	$.reset(select);
	$.append($$anchor, select);
	return $.pop($$exports);
}
