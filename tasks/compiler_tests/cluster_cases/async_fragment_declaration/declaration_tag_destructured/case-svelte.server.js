import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let source = {
		x: 1,
		y: 2
	};
	let x;
	let y;
	var promises = $$renderer.run([async () => ({x, y} = (await $.save(Promise.resolve(source)))())]);
	$$renderer.push(`<p>`);
	$$renderer.async([promises[0]], ($$renderer) => $$renderer.push(() => $.escape(x)));
	$$renderer.async([promises[0]], ($$renderer) => $$renderer.push(() => $.escape(y)));
	$$renderer.push(`</p> <button>go</button>`);
}
