import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { src, $$slots, $$events, ...rest } = $$props;
	$$renderer.push(`<embed${$.attributes({
		src,
		...rest
	})} onload="this.__e=event" onerror="this.__e=event"/>`);
}
