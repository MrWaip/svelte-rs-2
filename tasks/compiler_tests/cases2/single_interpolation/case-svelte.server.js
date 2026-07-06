import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<!---->${$.escape(some_varaible)}`);
}
