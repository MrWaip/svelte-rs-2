import * as $ from "svelte/internal/server";
function row($$renderer, text) {
	$$renderer.push(`<span>${$.escape(text)}</span>`);
}
export const KIND = "v1";
export function label(name) {
	return `${KIND}:${name}`;
}
export default function App($$renderer, $$props) {
	let { title } = $$props;
	row($$renderer, label(title));
}
