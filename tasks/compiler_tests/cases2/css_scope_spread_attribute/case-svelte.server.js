import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { $$slots, $$events, ...rest } = $$props;
	$$renderer.push(`<p${$.attributes({
		...rest,
		"data-extra": "x"
	}, "svelte-qv4ee3")}>spread</p>`);
}
