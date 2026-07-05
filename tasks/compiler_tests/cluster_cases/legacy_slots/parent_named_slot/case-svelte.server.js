import * as $ from "svelte/internal/server";
import A from "./A.svelte";
export default function App($$renderer) {
	A($$renderer, { $$slots: { s: ($$renderer) => {
		$$renderer.push(`<span slot="s">x</span>`);
	} } });
}
