import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { x } = $$props;
	{
		function prop($$renderer) {
			$$renderer.push(`<!---->${$.escape(foo)}`);
		}
		Comp($$renderer, {
			prop,
			children: ($$renderer) => {
				const foo = x * 2;
			},
			$$slots: {
				prop: true,
				default: true
			}
		});
	}
}
