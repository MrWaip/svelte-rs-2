import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let tag = "div";
	let myId = "foo";
	$.element($$renderer, tag, () => {
		$$renderer.push(` class="bar"${$.attr("id", myId)}`);
	}, () => {
		$$renderer.push(`content`);
	});
}
