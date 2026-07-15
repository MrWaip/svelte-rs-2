import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<div>`);
	$$renderer.push(`<style>
		.x {
			color: red;
		}
	</style>`);
	$$renderer.push(`</div>`);
}
