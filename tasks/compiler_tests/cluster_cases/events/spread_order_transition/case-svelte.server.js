import * as $ from "svelte/internal/server";
import { slide } from "svelte/transition";
export default function App($$renderer, $$props) {
	let rest = $.fallback($$props["rest"], () => ({}), true);
	$$renderer.push(`<div${$.attributes({ ...rest })}></div>`);
	$.bind_props($$props, { rest });
}
