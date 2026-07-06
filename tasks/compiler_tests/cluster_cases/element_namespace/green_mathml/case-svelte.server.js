import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<math><mi>x</mi></math>`);
}
