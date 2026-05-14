import * as $ from "svelte/internal/client";
import Child from "./Child.svelte";
var root = $.from_html(`<!> <!>`, 1);
export default function App($$anchor) {
	const binding_group = [];
	const binding_group_1 = [];
	let a = $.state("x");
	let b = $.state("y");
	var fragment = root();
	var node = $.first_child(fragment);
	Child(node, {
		get group() {
			return $.get(a);
		},
		set group($$value) {
			$.set(a, $$value, true);
		}
	});
	var node_1 = $.sibling(node, 2);
	Child(node_1, {
		get group() {
			return $.get(b);
		},
		set group($$value) {
			$.set(b, $$value, true);
		}
	});
	$.append($$anchor, fragment);
}
