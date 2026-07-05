import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { value = "", extra = {} } = $$props;
		$$renderer.push(`<textarea${$.attributes({ ...extra })}>`);
		const $$body = $.escape(value);
		if ($$body) {
			$$renderer.push(`${$$body}`);
		} else {}
		$$renderer.push(`</textarea>`);
		$.bind_props($$props, { value });
	});
}
