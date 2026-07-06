import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let tag = "div";
	let props = { id: "x" };
	$.element($$renderer, tag, () => {
		$$renderer.push(`${$.attributes({ ...props }, "svelte-16b9921")}`);
	}, () => {
		$$renderer.push(`x`);
	});
}
