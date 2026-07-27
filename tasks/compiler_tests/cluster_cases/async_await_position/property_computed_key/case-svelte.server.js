import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let x = 0;
		function delay(value) {
			return Promise.resolve(value);
		}
		function wrap(object) {
			return Object.keys(object).length;
		}
		$$renderer.push(`<button>inc</button> `);
		$$renderer.push(async () => $.escape(wrap({ [(await $.save(delay(x)))()]: 1 })));
	});
}
