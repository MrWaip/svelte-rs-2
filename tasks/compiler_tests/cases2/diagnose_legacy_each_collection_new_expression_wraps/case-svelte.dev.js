import "svelte/internal/flags/legacy";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<span></span>`), App[$.FILENAME], [[4, 4]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$exports = { ...$.legacy_api() };
	$.init();
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 0, () => $.untrack(() => new Array(4).fill(null)), $.index, ($$anchor, _, i) => {
		var span = root();
		span.textContent = i;
		$.append($$anchor, span);
	}), "each", App, 3, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
