import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let p = Promise.resolve({
		"a-b": 1,
		"c d": 2
	});
	$.await($$renderer, p, () => {}, ({ "a-b": ab, "c d": cd }) => {
		$$renderer.push(`<button>${$.escape(ab)}${$.escape(cd)}</button>`);
	});
	$$renderer.push(`<!--]-->`);
}
