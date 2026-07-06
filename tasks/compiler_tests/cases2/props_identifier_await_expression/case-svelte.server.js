import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		const { $$slots, $$events, ...props } = $$props;
		$.await($$renderer, fetch(1, 2, 3, props.field1), () => {}, () => {});
		$$renderer.push(`<!--]-->`);
	});
}
