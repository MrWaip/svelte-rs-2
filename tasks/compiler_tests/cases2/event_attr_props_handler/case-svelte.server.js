import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { onChange } = $$props;
	$$renderer.push(`<button>x</button>`);
}
