import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let active = false;
	async function getTag() {
		return "div";
	}
	$$renderer.child_block(async ($$renderer) => {
		$.element($$renderer, (await $.save(getTag()))(), () => {
			$$renderer.push(`${$.attr_class("", void 0, { "active": active })}`);
		}, () => {
			$$renderer.push(`x`);
		});
	});
}
