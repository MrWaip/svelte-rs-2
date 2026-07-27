App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let obj = $.tag_proxy($.proxy({ x: null }), "obj");
	let src = $.tag_proxy($.proxy({}), "src");
	let v = 0;
	var $$exports = { ...$.legacy_api() };
	var bind_get = () => v;
	var bind_set = (n) => obj.x = src;
	$.add_svelte_meta(() => Child($$anchor, {
		get value() {
			return bind_get();
		},
		set value($$value) {
			bind_set($$value);
		}
	}), "component", App, 8, 0, { componentTag: "Child" });
	return $.pop($$exports);
}
