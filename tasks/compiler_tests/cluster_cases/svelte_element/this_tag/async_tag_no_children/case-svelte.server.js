import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	async function getTag() {
		return "div";
	}
	$$renderer.child_block(async ($$renderer) => {
		$.element($$renderer, (await $.save(getTag()))());
	});
}
