import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { href } = $$props;
	$$renderer.push(`<svg><a${$.attr("xlink:href", href)}></a></svg>`);
}
