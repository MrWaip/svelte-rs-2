App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Test from "./Test.svelte";
var root = $.add_locations($.from_html(`<!> <!>`, 1), App[$.FILENAME], []);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let entries = $.tag_proxy($.proxy([]), "entries");
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var node = $.first_child(fragment);
	$.validate_binding("bind:this={entries[0]}", [], () => entries, () => 0, 6, 6);
	$.add_svelte_meta(() => $.bind_this(Test(node, {}), ($$value) => entries[0] = $$value, () => entries?.[0]), "component", App, 6, 0, { componentTag: "Test" });
	var node_1 = $.sibling(node, 2);
	$.add_svelte_meta(() => $.bind_this(Test(node_1, {}), (v) => entries[1] = v, () => entries[1]), "component", App, 7, 0, { componentTag: "Test" });
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
