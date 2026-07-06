import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let foo = $$props["foo"];
	$.head("q2w0q4", $$renderer, ($$renderer) => {
		$$renderer.push(`<meta name="description" content="A"/>`);
	});
	$$renderer.push(`<span>x</span> ${$.escape(foo)}`);
	$.bind_props($$props, { foo });
}
