import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let pending = null;
	let failed = () => {};
	$$renderer.boundary({ failed }, ($$renderer) => {
		if (pending) {
			$$renderer.push(`<!--[!-->`);
			pending($$renderer);
			$$renderer.push(`<!--]-->`);
		} else {
			$$renderer.push(`<!--[-->`);
			{
				$$renderer.push(`<!---->hi`);
			}
			$$renderer.push(`<!--]-->`);
		}
	});
}
