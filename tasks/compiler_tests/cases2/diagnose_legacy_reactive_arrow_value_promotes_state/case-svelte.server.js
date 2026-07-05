import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let handler;
		let onInput = $.fallback($$props["onInput"], () => {});
		let flag = false;
		$: handler = async (value) => {
			const result = await onInput(value, flag);
			if (result) {
				flag = true;
			}
		};
		$$renderer.push(`<button>go</button>`);
		$.bind_props($$props, { onInput });
	});
}
