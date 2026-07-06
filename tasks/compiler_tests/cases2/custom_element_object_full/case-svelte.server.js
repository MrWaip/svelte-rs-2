import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { count = 0 } = $$props;
	$$renderer.push(`<p>${$.escape(count)}</p>`);
}
