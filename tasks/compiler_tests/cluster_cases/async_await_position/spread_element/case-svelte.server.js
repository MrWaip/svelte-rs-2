import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let x = 0;
	function delay(value) {
		return Promise.resolve([value]);
	}
	function join(...args) {
		return args.length;
	}
	$$renderer.push(`<button>inc</button> `);
	$$renderer.push(async () => $.escape(join(...(await $.save(delay(x)))())));
	$$renderer.push(`
`);
	$$renderer.push(async () => $.escape(join([...(await $.save(delay(x)))()])));
}
