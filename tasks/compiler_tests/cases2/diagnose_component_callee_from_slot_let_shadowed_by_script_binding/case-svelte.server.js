import * as $ from "svelte/internal/server";
import Holder from "./Holder.svelte";
export default function App($$renderer) {
	async function load() {
		const { default: Inner } = await import("./Inner.svelte");
		return Inner;
	}
	Holder($$renderer, {
		task: load,
		children: $.invalid_default_snippet,
		$$slots: { default: ($$renderer, { value: Inner }) => {
			Inner($$renderer, {});
		} }
	});
}
