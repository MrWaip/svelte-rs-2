import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { error, fallback } = $$props;
	let status = $.derived(() => error ? "error" : fallback);
	$$renderer.push(`<span>${$.escape(status())}</span>`);
}
