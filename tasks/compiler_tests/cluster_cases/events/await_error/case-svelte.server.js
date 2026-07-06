import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let promise = Promise.reject();
	$.await($$renderer, promise, () => {}, () => {});
	$$renderer.push(`<!--]-->`);
}
