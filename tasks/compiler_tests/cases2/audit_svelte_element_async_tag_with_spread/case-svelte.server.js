import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let props = { id: "x" };
	async function getTag() {
		return "div";
	}
	$$renderer.child_block(async ($$renderer) => {
		$.element($$renderer, (await $.save(getTag()))(), () => {
			$$renderer.push(`${$.attributes({ ...props })}`);
		}, () => {
			$$renderer.push(`x`);
		});
	});
}
