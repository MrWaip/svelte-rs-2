App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
const recurse = $.wrap_snippet(App, function($$anchor) {
	$.validate_snippet_args(...arguments);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => App(node, {}), "component", App, 2, 1, { componentTag: "svelte:self" });
	$.append($$anchor, fragment);
});
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => recurse($$anchor), "render", App, 5, 0);
	return $.pop($$exports);
}
