App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let obj = $.tag_proxy($.proxy({ x: null }), "obj");
	let src = $.tag_proxy($.proxy({}), "src");
	let C = $.tag_proxy($.proxy(Child), "C");
	var $$exports = { ...$.legacy_api() };
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.component(node, () => C, ($$anchor, $$component) => {
		$$component($$anchor, { onChange: (v) => $.assign(obj, "x", "=", src, "(unknown):8:45") });
	}), "component", App, 8, 0, { componentTag: "svelte:component" });
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
