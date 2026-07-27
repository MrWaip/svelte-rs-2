import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	function fn() {
		return 1;
	}
	$$renderer.child(async ($$renderer) => {
		const $$0 = $.clsx((await $.save("neato"))());
		$$renderer.push(`<p${$.attributes({
			...{},
			class: $$0,
			id: fn()
		})}>x</p>`);
	});
}
