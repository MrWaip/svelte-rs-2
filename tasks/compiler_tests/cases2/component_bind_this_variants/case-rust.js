import * as $ from "svelte/internal/client";
import Component from "./Component.svelte";
var root = $.from_html(`<!> <!>`, 1);
export default function App($$anchor) {
	let items = [
		1,
		2,
		3
	];
	let refs = $.proxy([]);
	let obj = { ref: null };
	var fragment = root();
	var node = $.first_child(fragment);
	$.each(node, 17, () => items, $.index, ($$anchor, item, i) => {
		$.bind_this(Component($$anchor, {}), ($$value, i) => refs[i] = $$value, (i) => refs?.[i], () => [i]);
	});
	var node_1 = $.sibling(node, 2);
	$.bind_this(Component(node_1, {}), ($$value) => obj.ref = $$value, () => obj?.ref);
	$.append($$anchor, fragment);
}
