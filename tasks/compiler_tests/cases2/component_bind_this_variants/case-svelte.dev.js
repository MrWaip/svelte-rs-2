App[$.FILENAME] = "(unknown)";
import * as $ from "svelte/internal/client";
import Component from "./Component.svelte";
var root = $.add_locations($.from_html(`<!> <!>`, 1), App[$.FILENAME], []);
export default function App($$anchor, $$props) {
	$.check_target(new.target);
	$.push($$props, true, App);
	let items = [
		1,
		2,
		3
	];
	let refs = $.tag_proxy($.proxy([]), "refs");
	let obj = { ref: null };
	var $$exports = { ...$.legacy_api() };
	var fragment = root();
	var node = $.first_child(fragment);
	$.add_svelte_meta(() => $.each(node, 17, () => items, $.index, ($$anchor, item, i) => {
		$.validate_binding("bind:this={refs[i]}", [], () => refs, () => i, 9, 12);
		$.add_svelte_meta(() => $.bind_this(Component($$anchor, {}), ($$value, i) => refs[i] = $$value, (i) => refs?.[i], () => [i]), "component", App, 9, 1, { componentTag: "Component" });
	}), "each", App, 8, 0);
	var node_1 = $.sibling(node, 2);
	$.validate_binding("bind:this={obj.ref}", [], () => obj, () => "ref", 12, 11);
	$.add_svelte_meta(() => $.bind_this(Component(node_1, {}), ($$value) => obj.ref = $$value, () => obj?.ref), "component", App, 12, 0, { componentTag: "Component" });
	$.append($$anchor, fragment);
	return $.pop($$exports);
}
