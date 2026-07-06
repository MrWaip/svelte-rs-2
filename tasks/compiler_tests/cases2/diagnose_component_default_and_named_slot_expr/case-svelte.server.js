import * as $ from "svelte/internal/server";
import Inner from "./Inner.svelte";
export default function App($$renderer) {
	let counter = 0;
	Inner($$renderer, {
		children: ($$renderer) => {
			$$renderer.push(`<!---->default text`);
		},
		$$slots: {
			default: true,
			footer: ($$renderer) => {
				$$renderer.push(`<div slot="footer">Footer: 0</div>`);
			}
		}
	});
}
