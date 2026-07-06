import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	{
		function right($$renderer) {
			Btn($$renderer, {});
		}
		Header($$renderer, {
			right,
			$$slots: { right: true }
		});
	}
}
