import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let tag = "div";
	$.element($$renderer, tag, () => {
		$$renderer.push(` class="dynamic svelte-z15oen"`);
	}, () => {
		$$renderer.push(`content`);
	});
}
