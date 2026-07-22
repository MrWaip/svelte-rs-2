import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	C($$renderer, {
		children: ($$renderer) => {
			$$renderer.push(`<!---->${$.escape(x)}${$.escape(y)}`);
		},
		$$slots: {
			default: true,
			s: ($$renderer) => {
				$$renderer.push(`<a slot="s">1</a>`);
			}
		}
	});
}
