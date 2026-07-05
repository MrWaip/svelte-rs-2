import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let tag = "div";
	let props = { id: "bar" };
	let color = "red";
	$.element($$renderer, tag, () => {
		$$renderer.push(`${$.attributes({ ...props }, void 0, void 0, { color })}`);
	}, () => {
		$$renderer.push(`x`);
	});
}
