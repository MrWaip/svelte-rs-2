import * as $ from "svelte/internal/server";
import Outer from "./Outer.svelte";
export default function App($$renderer, $$props) {
	let { children } = $$props;
	Outer($$renderer, {
		children: ($$renderer) => {
			{
				children?.($$renderer);
				$$renderer.push(`<!---->`);
			}
		},
		$$slots: { default: true }
	});
}
