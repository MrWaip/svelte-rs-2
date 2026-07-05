import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let item = $.prop($$props, "item", 8);
	var $$exports = { ...$.legacy_api() };
	$.init();
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 1, () => ($.deep_read_state(item()), $.untrack(() => item().list)), $.index, ($$anchor, entry) => {
		$.next();
		var text = $.text();
		$.template_effect(() => $.set_text(text, $.get(entry)));
		$.append($$anchor, text);
	}), "each", App, 6, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
