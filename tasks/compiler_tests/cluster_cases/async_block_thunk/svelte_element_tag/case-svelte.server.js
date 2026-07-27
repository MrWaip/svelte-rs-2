import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let x = 0;
	function delay(value) {
		return Promise.resolve(value);
	}
	$$renderer.push(`<button>inc</button> `);
	$$renderer.child_block(async ($$renderer) => {
		$.element($$renderer, (await $.save(delay(x)))() + "div", void 0, () => {
			$$renderer.push(`content`);
		});
	});
}
