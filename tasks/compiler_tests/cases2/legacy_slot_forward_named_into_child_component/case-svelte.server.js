import * as $ from "svelte/internal/server";
import Inner from "./Inner.svelte";
export default function App($$renderer, $$props) {
	Inner($$renderer, { $$slots: {
		icon: ($$renderer) => {
			$$renderer.push(`<!--[-->`);
			$.slot($$renderer, $$props, "icon", {}, null);
			$$renderer.push(`<!--]-->`);
		},
		caption: ($$renderer) => {
			$$renderer.push(`<!--[-->`);
			$.slot($$renderer, $$props, "caption", {}, null);
			$$renderer.push(`<!--]-->`);
		}
	} });
}
