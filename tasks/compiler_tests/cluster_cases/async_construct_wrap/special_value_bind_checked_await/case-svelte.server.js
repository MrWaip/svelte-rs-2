import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { a } = $$props;
	let g = void 0;
	$$renderer.child(async ($$renderer) => {
		const $$0 = (await $.save(a))();
		$$renderer.push(`<input type="checkbox"${$.attr("checked", g, true)}${$.attr("value", $$0)}/>`);
	});
}
