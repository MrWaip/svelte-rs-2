import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
import Child from "./Child.svelte";
export default function App($$renderer, $$props) {
	async function f() {
		return 1;
	}
	async function g() {
		return 2;
	}
	let { tag } = $$props;
	$$renderer.child_block(async ($$renderer) => {
		const $$0 = (await $.save(f()))();
		Child($$renderer, {
			a: $$0,
			children: ($$renderer) => {
				$$renderer.child(async ($$renderer) => {
					const $$0 = (await $.save(g()))();
					$.element($$renderer, tag, () => {
						$$renderer.push(`${$.attr("title", $$0)}`);
					});
				});
			},
			$$slots: { default: true }
		});
	});
}
