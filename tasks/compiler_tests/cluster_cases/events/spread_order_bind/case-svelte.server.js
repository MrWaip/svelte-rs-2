import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let rest = $.fallback($$props["rest"], () => ({}), true);
	let value = $.fallback($$props["value"], "");
	$$renderer.push(`<input${$.attributes({
		...rest,
		value
	}, void 0, void 0, void 0, 4)}/>`);
	$.bind_props($$props, {
		rest,
		value
	});
}
