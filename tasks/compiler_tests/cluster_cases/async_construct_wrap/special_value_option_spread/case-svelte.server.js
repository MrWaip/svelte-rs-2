import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { rest, a } = $$props;
	$$renderer.child(async ($$renderer) => {
		const $$0 = (await $.save(a))();
		$$renderer.option({ ...rest }, $$0);
	});
}
