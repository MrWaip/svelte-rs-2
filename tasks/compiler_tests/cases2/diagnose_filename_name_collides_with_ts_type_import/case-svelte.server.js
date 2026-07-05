import * as $ from "svelte/internal/server";
export default function DepositMethod($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { $$slots, $$events, ...props } = $$props;
		$$renderer.push(`<p>${$.escape(props.title)}</p>`);
	});
}
