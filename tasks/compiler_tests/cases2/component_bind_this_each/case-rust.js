import * as $ from "svelte/internal/client";
import Component from "./Component.svelte";
export default function App($$anchor) {
	let items = [
		1,
		2,
		3
	];
	let refs = $.proxy([]);
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.each(node, 17, () => items, $.index, ($$anchor, item, i) => {
		$.bind_this(Component($$anchor, {}), ($$value, i) => refs[i] = $$value, (i) => refs?.[i], () => [i]);
	});
	$.append($$anchor, fragment);
}
