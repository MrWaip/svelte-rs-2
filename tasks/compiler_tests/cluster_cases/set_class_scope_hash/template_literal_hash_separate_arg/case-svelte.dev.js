App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div>a</div>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var div = root();
	let classes;
	$.template_effect(() => classes = $.set_class(div, 1, `title`, "svelte-95v2c0", classes, { active: $$props.x }));
	$.append($$anchor, div);
	return $.pop($$exports);
}
