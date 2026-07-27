import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let deferred = Promise.withResolvers();
	$$renderer.push(`<ul><!--[-->`);
	$$renderer.child_block(async ($$renderer) => {
		const each_array = $.ensure_array_like((await $.save(deferred.promise))());
		for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
			let item = each_array[$$index];
			$$renderer.push(`<li>${$.escape(item)}</li>`);
		}
	});
	$$renderer.push(`<!--]--></ul>`);
}
