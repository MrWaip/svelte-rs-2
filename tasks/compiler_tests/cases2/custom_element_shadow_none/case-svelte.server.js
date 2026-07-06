import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { name } = $$props;
	$$renderer.push(`<p>${$.escape(name)}</p>`);
}
