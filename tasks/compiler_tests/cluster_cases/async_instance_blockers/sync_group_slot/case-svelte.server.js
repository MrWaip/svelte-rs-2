import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let gate = 0;
	var one, sync1, sync2, two, sync3;
	var $$promises = $$renderer.run([
		async () => one = await $.async_derived(() => gate),
		() => {
			sync1 = gate + 1;
			sync2 = gate + 2;
		},
		async () => two = await $.async_derived(() => gate),
		() => sync3 = gate + 3
	]);
	$$renderer.push(`<button>inc</button> <p>`);
	$$renderer.async([$$promises[0]], ($$renderer) => $$renderer.push(() => $.escape(one())));
	$$renderer.push(`</p> <p>`);
	$$renderer.async([$$promises[1]], ($$renderer) => $$renderer.push(() => $.escape(sync1)));
	$$renderer.async([$$promises[1]], ($$renderer) => $$renderer.push(() => $.escape(sync2)));
	$$renderer.push(`</p> <p>`);
	$$renderer.async([$$promises[2]], ($$renderer) => $$renderer.push(() => $.escape(two())));
	$$renderer.push(`</p> <p>`);
	$$renderer.async([$$promises[3]], ($$renderer) => $$renderer.push(() => $.escape(sync3)));
	$$renderer.push(`</p>`);
}
