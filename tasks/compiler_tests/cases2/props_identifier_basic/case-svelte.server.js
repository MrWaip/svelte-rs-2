import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		const { $$slots, $$events, ...props } = $$props;
		$$renderer.push(`<p>${$.escape(props.name)}</p>`);
	});
}
