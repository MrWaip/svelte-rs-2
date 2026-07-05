import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	const { x = 0 } = $$props;
	$$renderer.push(`<p>${$.escape(x)}</p>`);
}
