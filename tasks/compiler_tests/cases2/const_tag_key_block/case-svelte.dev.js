App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p></p>`), App[$.FILENAME], [[7, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let count = 1;
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.key(node, () => count, ($$anchor) => {
		const doubled = $.tag($.derived(() => count * 2), "doubled");
		$.get(doubled);
		var p = root();
		p.textContent = $.get(doubled);
		$.append($$anchor, p);
	}), "key", App, 5, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
