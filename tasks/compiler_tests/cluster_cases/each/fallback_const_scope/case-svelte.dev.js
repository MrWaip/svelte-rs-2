import "svelte/internal/flags/legacy";
import "svelte/internal/flags/async";
App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<!> <!>`, 1), App[$.FILENAME], []);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, false, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 0, () => ({ length: 1 }), $.index, ($$anchor, $$item) => {
		const data = $.tag($.derived_safe_equal(() => 1), "data");
		$.get(data);
		$.next();
		var text = $.text();
		text.nodeValue = $.get(data);
		$.append($$anchor, text);
	}), "each", App, 1, 0);
	var node_1 = $.sibling(node, 2);
	$.add_svelte_meta(() => $.each(node_1, 0, () => ({ length: 0 }), $.index, ($$anchor, $$item) => {
		$.next();
		var text_1 = $.text("x");
		$.append($$anchor, text_1);
	}, ($$anchor) => {
		const data = $.tag($.derived_safe_equal(() => 2), "data");
		$.get(data);
		$.next();
		var text_2 = $.text();
		text_2.nodeValue = $.get(data);
		$.append($$anchor, text_2);
	}), "each", App, 6, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
