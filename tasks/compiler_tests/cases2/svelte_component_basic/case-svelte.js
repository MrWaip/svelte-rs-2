import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
import A from "./A.svelte";
export default function App($$anchor) {
	let current = A;
	var fragment = $.comment();
	var node = $.first_child(fragment);
	$.component(node, () => current, ($$anchor, $$component) => {
		$$component($$anchor, { answer: 42 });
	});
	$.append($$anchor, fragment);
}
