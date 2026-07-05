App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let show = $.prop($$props, "show", 3, null);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.snippet(node, () => show() ?? $.noop, () => "hello"), "render", App, 5, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
