App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div>_</div> `, 1), App[$.FILENAME], [[8, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let title = $.tag($.state(10), "title");
	let title2 = $.tag($.state(12), "title2");
	$.update(title, -1);
	$.update_pre(title2);
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var div = $.first_child(fragment);
	var text = $.sibling(div);
	$.template_effect(() => {
		$.set_attribute(div, "attr", $.update(title));
		$.set_text(text, ` ${$.update_pre(title2, -1) ?? ""}`);
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
