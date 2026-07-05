import * as $ from "svelte/internal/server";
import Inner from "./Inner.svelte";
import Button from "./Button.svelte";
export default function App($$renderer) {
	Inner($$renderer, {
		children: $.invalid_default_snippet,
		$$slots: { default: ($$renderer, { onClick }) => {
			Button($$renderer, {});
		} }
	});
}
