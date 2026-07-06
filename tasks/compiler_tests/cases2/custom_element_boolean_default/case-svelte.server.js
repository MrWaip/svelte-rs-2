import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { active = false } = $$props;
	$$renderer.push(`<p>${$.escape(active)}</p>`);
}
