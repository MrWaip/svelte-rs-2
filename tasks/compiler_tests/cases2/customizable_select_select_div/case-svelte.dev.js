App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var select_content = $.add_locations($.from_html(`<div>Rich</div>`, 1), App[$.FILENAME], [[4, 8]]);
var root = $.add_locations($.from_html(`<select><!></select>`), App[$.FILENAME], [[4, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var select = root();
	$.customizable_select(select, () => {
		var anchor = $.child(select);
		var fragment = select_content();
		$.append(anchor, fragment);
	});
	$.append($$anchor, select);
	return $.pop($$exports);
}
