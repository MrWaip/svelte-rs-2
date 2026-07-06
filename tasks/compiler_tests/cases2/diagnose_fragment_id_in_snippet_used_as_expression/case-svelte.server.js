import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { $$slots, $$events, ...props } = $$props;
		function body($$renderer) {
			if (props.Inner) {
				$$renderer.push("<!--[0-->");
				if (props.Inner) {
					$$renderer.push("<!--[-->");
					props.Inner($$renderer, {});
					$$renderer.push("<!--]-->");
				} else {
					$$renderer.push("<!--[!-->");
					$$renderer.push("<!--]-->");
				}
			} else {
				$$renderer.push("<!--[-1-->");
			}
			$$renderer.push(`<!--]-->`);
		}
		Child($$renderer, { icon: props.show ? body : undefined });
	});
}
