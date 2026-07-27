import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	async function h() {
		return 1;
	}
	async function f() {
		return 2;
	}
	$$renderer.child(async ($$renderer) => {
		const [$$0, $$1] = (await $.save(Promise.all([(async () => (await $.save(h()))())(), (async () => (await $.save(f()))())()])))();
		$$renderer.push(`<div${$.attr("title", `z${$.stringify($$0)}`)}${$.attr_class("", void 0, { "a": $$1 })}></div>`);
	});
}
