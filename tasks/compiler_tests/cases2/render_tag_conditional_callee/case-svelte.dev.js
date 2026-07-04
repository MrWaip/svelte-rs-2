App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.snippet(node, () => $$props.cond ? $$props.a : $$props.b), "render", App, 5, 0);
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
