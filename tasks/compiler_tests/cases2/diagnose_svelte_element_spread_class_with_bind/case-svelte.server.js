import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let tag = "div";
	let props = { id: "bar" };
	let active = false;
	let el = void 0;
	$.element($$renderer, tag, () => {
		$$renderer.push(`${$.attributes({ ...props }, void 0, { active })}`);
	}, () => {
		$$renderer.push(`x`);
	});
}
