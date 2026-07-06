import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let props = { foo: "bar" };
	let item = "hello";
	let extra = "world";
	$$renderer.push(`<!--[-->`);
	$.slot($$renderer, $$props, "footer", $.spread_props([{
		item,
		extra
	}, props]), null);
	$$renderer.push(`<!--]-->`);
}
