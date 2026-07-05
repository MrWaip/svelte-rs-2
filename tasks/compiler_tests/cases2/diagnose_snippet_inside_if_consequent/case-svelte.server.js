import * as $ from "svelte/internal/server";
import Outer from "./Outer.svelte";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { data } = $$props;
		{
			function body($$renderer) {
				if (data?.flag) {
					$$renderer.push("<!--[0-->");
					function inner($$renderer) {
						$$renderer.push(`<!---->${$.escape(data?.flag?.text)}`);
					}
					$$renderer.push(`<div></div>`);
				} else {
					$$renderer.push("<!--[-1-->");
				}
				$$renderer.push(`<!--]-->`);
			}
			Outer($$renderer, {
				body,
				$$slots: { body: true }
			});
		}
	});
}
