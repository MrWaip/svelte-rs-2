import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { manager } = $$props;
	let a = $.derived(() => manager.a), b = $.derived(() => manager.b);
	$$renderer.push(`<p>${$.escape(a())},${$.escape(b())}</p>`);
}
