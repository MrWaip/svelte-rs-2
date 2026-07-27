import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { tag } = $$props;
	async function g() {
		return 2;
	}
	$$renderer.child(async ($$renderer) => {
		const $$0 = (await $.save(g()))();
		$.element($$renderer, tag, () => {
			$$renderer.push(`${$.attr("title", $$0)}`);
		});
	});
}
