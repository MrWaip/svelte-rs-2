import * as $ from "svelte/internal/server";
import A from "./A.svelte";
export default function App($$renderer) {
	A($$renderer, {
		children: "foo",
		$$slots: { default: ($$renderer) => {
			$$renderer.push(`<!---->bar`);
		} }
	});
}
