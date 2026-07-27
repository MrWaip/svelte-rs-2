import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.child(async ($$renderer) => {
		const $$0 = $.clsx((await $.save("a"))());
		$$renderer.push(`<div${$.attr_class($$0, void 0, { "b": true })}>y</div>`);
	});
}
