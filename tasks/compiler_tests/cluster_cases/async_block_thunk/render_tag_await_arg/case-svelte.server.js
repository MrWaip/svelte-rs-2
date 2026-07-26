import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
function row($$renderer, value) {
	$$renderer.push(`<p>${$.escape(value)}</p>`);
}
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let x = 0;
		function delay(value) {
			return Promise.resolve({ value });
		}
		$$renderer.push(`<button>inc</button> `);
		$$renderer.child_block(async ($$renderer) => {
			const $$0 = (await $.save((await $.save(delay(x)))().value))();
			row($$renderer, $$0);
		});
	});
}
