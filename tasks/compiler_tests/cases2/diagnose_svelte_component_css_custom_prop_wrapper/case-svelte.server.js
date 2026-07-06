import * as $ from "svelte/internal/server";
import Icon from "./Icon.svelte";
export default function App($$renderer, $$props) {
	let { color } = $$props;
	let current = Icon;
	$$renderer.push(`<span class="wrap svelte-1fxeua7">`);
	$.css_props($$renderer, true, { "--my-color": `var(--${$.stringify(color)})` }, () => {
		if (current) {
			$$renderer.push("<!--[-->");
			current($$renderer, {});
			$$renderer.push("<!--]-->");
		} else {
			$$renderer.push("<!--[!-->");
			$$renderer.push("<!--]-->");
		}
	}, true);
	$$renderer.push(`</span>`);
}
