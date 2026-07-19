import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { n } = $$props;
	const doubled = n * 2;
	$$renderer.push(`<p>${$.escape(doubled)}</p>`);
}
