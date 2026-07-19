import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { title = "a" + "b" + "c" } = $$props;
	$$renderer.push(`<p>${$.escape(title)}</p>`);
}
