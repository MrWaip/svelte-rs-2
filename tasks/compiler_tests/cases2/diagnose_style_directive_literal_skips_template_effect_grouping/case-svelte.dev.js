import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div> </div>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let item = $.prop($$props, "item", 8);
	var $$exports = { ...$.legacy_api() };
	$.init();
	var div = root();
	$.set_style(div, "", {}, { display: "flex" });
	var text = $.child(div, true);
	$.reset(div);
	$.template_effect(() => $.set_text(text, ($.deep_read_state(item()), $.untrack(() => item().text))));
	$.append($$anchor, div);
	return $.pop($$exports);
}
