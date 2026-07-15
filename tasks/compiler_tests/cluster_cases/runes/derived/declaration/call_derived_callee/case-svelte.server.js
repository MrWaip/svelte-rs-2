import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { config } = $$props;
	const makeStore = $.derived(() => config.makeStore);
	const entries = $.derived(() => makeStore()());
	$$renderer.push(`<span>${$.escape(entries().x)}</span>`);
}
