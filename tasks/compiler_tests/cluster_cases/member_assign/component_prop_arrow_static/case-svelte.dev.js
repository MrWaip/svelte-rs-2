App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let obj = $.tag_proxy($.proxy({ x: null }), "obj");
	let src = $.tag_proxy($.proxy({}), "src");
	var $$exports = { ...$.legacy_api() };
	$.add_svelte_meta(() => Child($$anchor, { onChange: (v) => obj.x = src }), "component", App, 7, 0, { componentTag: "Child" });
	return $.pop($$exports);
}
