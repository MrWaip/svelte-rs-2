import * as $ from "svelte/internal/server";
import { createBubbler } from "svelte/legacy";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let rest = $.fallback($$props["rest"], () => ({}), true);
		const bubbler = createBubbler();
		$$renderer.push(`<div${$.attributes({ ...rest })}></div>`);
		$.bind_props($$props, { rest });
	});
}
