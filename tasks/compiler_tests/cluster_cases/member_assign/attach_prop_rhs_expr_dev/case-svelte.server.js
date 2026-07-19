import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { opacity = .5 } = $$props;
	$$renderer.push(`<div></div>`);
}
