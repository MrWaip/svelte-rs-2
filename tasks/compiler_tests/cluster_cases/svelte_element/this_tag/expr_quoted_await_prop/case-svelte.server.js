import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { pending } = $$props;
	$$renderer.child_block(async ($$renderer) => {
		$.element($$renderer, (await $.save(pending))(), void 0, () => {
			$$renderer.push(`hello`);
		});
	});
}
