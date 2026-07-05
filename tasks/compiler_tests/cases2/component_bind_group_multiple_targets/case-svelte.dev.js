App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
var root = $.add_locations($.from_html(`<!> <!>`, 1), App[$.FILENAME], []);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	const binding_group = [];
	const binding_group_1 = [];
	let a = $.tag($.state("x"), "a");
	let b = $.tag($.state("y"), "b");
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => Child(node, {
		get group() {
			return $.get(a);
		},
		set group($$value) {
			$.set(a, $$value, true);
		}
	}), "component", App, 7, 0, { componentTag: "Child" });
	var node_1 = $.sibling(node, 2);
	$.add_svelte_meta(() => Child(node_1, {
		get group() {
			return $.get(b);
		},
		set group($$value) {
			$.set(b, $$value, true);
		}
	}), "component", App, 8, 0, { componentTag: "Child" });
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
