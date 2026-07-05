import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { width = 1, alt, $$slots, $$events, ...rest } = $$props;
	$$renderer.push(`<div${$.attributes({
		width,
		alt,
		...rest
	})}></div>`);
}
