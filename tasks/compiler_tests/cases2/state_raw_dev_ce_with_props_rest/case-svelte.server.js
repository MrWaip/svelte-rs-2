import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { x = 0, $$slots, $$events, ...rest } = $$props;
	let rawData = {
		a: 1,
		b: 2
	};
	let snapshot = rawData;
	$$renderer.push(`<p>${$.escape(x)} ${$.escape(snapshot.a)}</p>`);
}
