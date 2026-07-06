import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let foo = $$props["foo"];
	$$renderer.push(`<!---->${$.escape(foo)}`);
	$.bind_props($$props, { foo });
}
