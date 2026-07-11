App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div>a</div>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var div = root();
	let classes;
	$.template_effect(() => classes = $.set_class(div, 1, "svelte-c546ri", null, classes, { active: $$props.on }));
	$.append($$anchor, div);
	return $.pop($$exports);
}
