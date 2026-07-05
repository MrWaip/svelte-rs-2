import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<div> </div>`), App[$.FILENAME], [[8, 4]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let item = $.prop($$props, "item", 8);
	var $$exports = { ...$.legacy_api() };
	$.init();
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.key(node, () => ($.deep_read_state(item()), $.untrack(() => item().id)), ($$anchor) => {
		var div = root();
		var text = $.child(div, true);
		$.reset(div);
		$.template_effect(() => $.set_text(text, ($.deep_read_state(item()), $.untrack(() => item().id))));
		$.append($$anchor, div);
	}), "key", App, 7, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
