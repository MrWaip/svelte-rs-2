import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p> <p> </p>`, 1), App[$.FILENAME], [[7, 0], [8, 0]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let items = $.prop($$props, "items", 25, () => [1, 2]);
	let user = $.prop($$props, "user", 25, () => ({ name: "a" }));
	var $$exports = { ...$.legacy_api() };
	$.init(true);
	var fragment = root();
	var p = $.first_child(fragment);
	var text = $.child(p, true);
	$.reset(p);
	var p_1 = $.sibling(p, 2);
	var text_1 = $.child(p_1, true);
	$.reset(p_1);
	$.template_effect(() => {
		$.set_text(text, ($.deep_read_state(items()), $.untrack(() => items().length)));
		$.set_text(text_1, ($.deep_read_state(user()), $.untrack(() => user().name)));
	});
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
