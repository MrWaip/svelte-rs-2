import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let id = $.fallback($$props["id"], 1);
	async function fetchData(arg) {
		return arg;
	}
	$.await($$renderer, fetchData(id), () => {}, (v) => {
		$$renderer.push(`<span>${$.escape(v)}</span>`);
	});
	$$renderer.push(`<!--]-->`);
	$.bind_props($$props, { id });
}
