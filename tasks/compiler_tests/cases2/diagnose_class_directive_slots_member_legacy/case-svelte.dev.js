import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div><!> </div>`), App[$.FILENAME], [[5, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	const $$slots = $.sanitize_slots($$props);
	$.push($$props, false, App);
	let x = $.prop($$props, "x", 8);
	var $$exports = { ...$.legacy_api() };
	var div = root();
	$.set_class(div, 1, "", null, {}, { "before-content": $$slots.beforeContent });
	var node = $.child(div);
	$.slot(node, $$props, "beforeContent", {}, null);
	var text = $.sibling(node);
	$.reset(div);
	$.template_effect(() => $.set_text(text, ` ${x() ?? ""}`));
	$.append($$anchor, div);
	return $.pop($$exports);
}
