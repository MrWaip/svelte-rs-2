import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let x = 0;
	function delay(value) {
		return Promise.resolve(value);
	}
	$$renderer.push(`<button>inc</button> `);
	$$renderer.child(async ($$renderer) => {
		const $$0 = (await $.save(delay({ title: x })))();
		$$renderer.push(`<div${$.attributes({ ...$$0 })}></div>`);
	});
}
