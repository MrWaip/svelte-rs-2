import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let x = 0;
	function delay(value) {
		return Promise.resolve(value);
	}
	$$renderer.push(`<button>inc</button> `);
	$$renderer.child_block(async ($$renderer) => {
		$$renderer.push($.html((await $.save(delay(x)))() + "<b>bold</b>"));
	});
}
