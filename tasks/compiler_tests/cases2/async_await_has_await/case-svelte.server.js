import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	async function getPromise() {
		return fetch("/api");
	}
	$$renderer.child_block(async ($$renderer) => {
		$.await($$renderer, (async () => (await $.save(getPromise()))())(), () => {}, (value) => {
			$$renderer.push(`<p>${$.escape(value)}</p>`);
		});
	});
	$$renderer.push(`<!--]-->`);
}
