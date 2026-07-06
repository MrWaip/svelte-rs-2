import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	function tooltip(node, arg) {}
	var data, config;
	var $$promises = $$renderer.run([async () => data = await fetch("/api"), () => config = data.config]);
	$$renderer.push(`<div>hello</div>`);
}
