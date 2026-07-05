import * as $ from "svelte/internal/server";
import Outer from "./Outer.svelte";
export default function App($$renderer) {
	Outer($$renderer, { $$slots: { footer: ($$renderer) => {
		App($$renderer, { slot: "footer" });
		$$renderer.push(`<!---->`);
	} } });
}
