App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div>hello</div>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	const id = $.props_id();
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.template_effect(() => $.set_attribute(div, "id", id));
	$.append($$anchor, div);
	return $.pop($$exports);
}
