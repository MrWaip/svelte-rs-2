App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div> </div>`), App[$.FILENAME], [[6, 0]]);
export default function App($$anchor, $$props) {
	const id = $.props_id();
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var div = root();
	var text = $.child(div, true);
	$.reset(div);
	$.template_effect(() => {
		$.set_attribute(div, "id", id);
		$.set_text(text, $$props.name);
	});
	$.append($$anchor, div);
	return $.pop($$exports);
}
