import * as $ from "svelte/internal/server";
function greeting($$renderer, { label, name = "world" }) {
	$$renderer.push(`<p>${$.escape(label)}: ${$.escape(name)}</p>`);
}
export default function App($$renderer) {
	greeting($$renderer, { label: "Hi" });
}
