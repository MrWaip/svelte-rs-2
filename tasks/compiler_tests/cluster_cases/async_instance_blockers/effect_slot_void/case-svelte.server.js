import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		var $$promises = $$renderer.run([() => 1, () => void void 0]);
	});
}
