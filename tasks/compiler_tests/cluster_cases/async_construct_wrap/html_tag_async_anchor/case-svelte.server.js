import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let deferred = Promise.withResolvers();
	$$renderer.push(`<h1>`);
	$$renderer.child_block(async ($$renderer) => {
		$$renderer.push($.html((await $.save(deferred.promise))()));
	});
	$$renderer.push(`</h1>`);
}
