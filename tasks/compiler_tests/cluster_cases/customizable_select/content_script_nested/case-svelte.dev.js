App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var select_content = $.add_locations($.with_script($.from_html(`<option>A</option><div><script>console.log('hi')<\/script><!></div>`, 1)), App[$.FILENAME], [[2, 1], [
	3,
	1,
	[[3, 6]]
]]);
var root = $.add_locations($.from_html(`<select><!></select>`), App[$.FILENAME], [[1, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var select = root();
	$.customizable_select(select, () => {
		var anchor = $.child(select);
		var fragment = select_content();
		var option = $.first_child(fragment);
		option.value = option.__value = "a";
		$.next();
		$.append(anchor, fragment);
	});
	$.append($$anchor, select);
	return $.pop($$exports);
}
