import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	let items = $.prop($$props, "items", 8);
	var $$exports = { ...$.legacy_api() };
	$.head("q2w0q4", ($$anchor) => {
		var fragment = $.comment();
		var node = $.first_child(fragment);
		$.add_svelte_meta(() => $.each(node, 1, items, $.index, ($$anchor, i) => {
			$.deferred_template_effect(() => {
				$.document.title = $.get(i) ?? "";
			});
		}), "each", App, 6, 1);
		$.append($$anchor, fragment);
	});
	return $.pop($$exports);
}
