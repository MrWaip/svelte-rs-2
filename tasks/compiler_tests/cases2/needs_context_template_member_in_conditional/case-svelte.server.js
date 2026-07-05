import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { foo } = $$props;
		$$renderer.push(`<p>${$.escape(foo.bar ? "a" : "b")}</p>`);
	});
}
