import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { a, c } = $$props;
	$$renderer.child(async ($$renderer) => {
		const [$$0, $$1] = (await $.save(Promise.all([(async () => (await $.save(a))())(), (async () => (await $.save(c))())()])))();
		$$renderer.option({ class: $$1 }, $$0);
	});
}
