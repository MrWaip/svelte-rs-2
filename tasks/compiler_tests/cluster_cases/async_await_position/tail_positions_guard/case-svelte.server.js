import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let x = 0;
	function delay(value) {
		return Promise.resolve(value);
	}
	function pick(first, second) {
		return first + second;
	}
	function wrap(object) {
		return object.value;
	}
	$$renderer.push(`<button>inc</button> `);
	$$renderer.push(async () => $.escape(pick(1, await delay(x))));
	$$renderer.push(`
`);
	$$renderer.push(async () => $.escape(x > 0 ? await delay(x) : 0));
	$$renderer.push(`
`);
	$$renderer.push(async () => $.escape(`value ${await delay(x)}`));
	$$renderer.push(`
`);
	$$renderer.push(async () => $.escape((0, await delay(x))));
	$$renderer.push(`
`);
	$$renderer.push(async () => $.escape(wrap({
		index: 1,
		value: await delay(x)
	})));
}
