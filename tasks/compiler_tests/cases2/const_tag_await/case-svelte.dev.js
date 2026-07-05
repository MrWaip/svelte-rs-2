App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
var root = $.add_locations($.from_html(`<p> </p>`), App[$.FILENAME], [[7, 1]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let promise = $.tag_proxy($.proxy(Promise.resolve(42)), "promise");
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.await(node, () => promise, null, ($$anchor, value) => {
		const doubled = $.tag($.derived(() => $.get(value) * 2), "doubled");
		$.get(doubled);
		var p = root();
		var text = $.child(p, true);
		$.reset(p);
		$.template_effect(() => $.set_text(text, $.get(doubled)));
		$.append($$anchor, p);
	}), "await", App, 5, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
