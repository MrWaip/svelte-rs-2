import * as $ from "svelte/internal/server";
import Holder from "./Holder.svelte";
export default function App($$renderer) {
	Holder($$renderer, {
		children: $.invalid_default_snippet,
		$$slots: { default: ($$renderer, { value: Inner }) => {
			Inner($$renderer, {});
		} }
	});
}
