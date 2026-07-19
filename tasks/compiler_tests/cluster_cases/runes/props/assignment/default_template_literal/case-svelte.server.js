import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { label = `untitled` } = $$props;
	$$renderer.push(`<!---->${$.escape(label)}`);
}
