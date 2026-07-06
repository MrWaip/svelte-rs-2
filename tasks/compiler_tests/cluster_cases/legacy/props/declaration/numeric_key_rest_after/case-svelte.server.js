import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { 0: zero, $$slots, $$events, ...rest } = $$props;
		$$renderer.push(`<!---->${$.escape(zero)} ${$.escape(rest.foo)}`);
	});
}
