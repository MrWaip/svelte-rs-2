import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { count: total = 0 } = $$props;
	$$renderer.push(`<p>${$.escape(total)}</p>`);
}
