import * as $ from "svelte/internal/server";
import Test from "./Test.svelte";
export default function App($$renderer) {
	let entries = [];
	Test($$renderer, {});
	$$renderer.push(`<!----> `);
	Test($$renderer, {});
	$$renderer.push(`<!---->`);
}
