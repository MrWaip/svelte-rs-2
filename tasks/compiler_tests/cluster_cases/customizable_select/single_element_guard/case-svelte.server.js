import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { label } = $$props;
	$$renderer.push(`<select><div>${$.escape(label)}</div><!></select>`);
}
