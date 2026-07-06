import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		const { $$slots, $$events, ...props } = $$props;
		let title = $.derived(() => props?.title);
		$$renderer.push(`<!---->${$.escape(title())}`);
	});
}
