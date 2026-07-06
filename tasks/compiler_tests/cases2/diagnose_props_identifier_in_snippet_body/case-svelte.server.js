import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { $$slots, $$events, ...props } = $$props;
	function foo($$renderer) {
		if (props.X) {
			$$renderer.push("<!--[-->");
			props.X($$renderer, {});
			$$renderer.push("<!--]-->");
		} else {
			$$renderer.push("<!--[!-->");
			$$renderer.push("<!--]-->");
		}
	}
}
