import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { data, $$slots, $$events, ...rest } = $$props;
	$$renderer.push(`<object${$.attributes({
		data,
		...rest
	})} onload="this.__e=event" onerror="this.__e=event"></object>`);
}
