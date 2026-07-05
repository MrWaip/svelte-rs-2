App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
const icon = $.wrap_snippet(App, function($$anchor) {
	$.validate_snippet_args(...arguments);
	var span = root();
	$.append($$anchor, span);
});
var root = $.add_locations($.from_html(`<span>hi</span>`), App[$.FILENAME], [[5, 17]]);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Child($$anchor, { get icon() {
		return icon;
	} }), "component", App, 7, 0, { componentTag: "Child" });
	return $.pop($$exports);
}
