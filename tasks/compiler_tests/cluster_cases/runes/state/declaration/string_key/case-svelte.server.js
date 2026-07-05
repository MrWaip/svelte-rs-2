import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let tmp = {
		"a-b": 1,
		"c d": 2
	}, ab = tmp["a-b"], cd = tmp["c d"];
	$$renderer.push(`<button>${$.escape(ab)}${$.escape(cd)}</button>`);
}
