import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let deferred = Promise.withResolvers();
	$$renderer.child_block(async ($$renderer) => {
		$.element($$renderer, (await $.save(deferred.promise))(), void 0, () => {
			$$renderer.push(`hello`);
		});
	});
}
