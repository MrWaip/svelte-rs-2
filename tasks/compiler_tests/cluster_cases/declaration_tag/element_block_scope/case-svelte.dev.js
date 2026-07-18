App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div> </div>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	const active = $.strict_equals($$props.cls, "on");
	var div = root();
	var text = $.child(div, true);
	$.reset(div);
	$.template_effect(() => {
		$.set_class(div, 1, $.clsx(active ? "a" : "b"));
		$.set_text(text, active);
	});
	$.append($$anchor, div);
	return $.pop($$exports);
}
