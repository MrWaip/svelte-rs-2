import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { x, y = 10 } = $$props;
	$$renderer.push(`<p>${$.escape(x)}</p> <p>${$.escape(y)}</p>`);
}
