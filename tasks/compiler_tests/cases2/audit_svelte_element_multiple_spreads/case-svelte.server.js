import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let tag = "div";
	let a = { id: "a" };
	let b = { id: "b" };
	$.element($$renderer, tag, () => {
		$$renderer.push(`${$.attributes({
			...a,
			...b
		})}`);
	}, () => {
		$$renderer.push(`x`);
	});
}
