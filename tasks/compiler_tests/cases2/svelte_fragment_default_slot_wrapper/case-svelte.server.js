import * as $ from "svelte/internal/server";
import Outer from "./Outer.svelte";
export default function App($$renderer, $$props) {
	let name = $$props["name"];
	Outer($$renderer, {
		children: ($$renderer) => {
			{
				$$renderer.push(`hello ${$.escape(name)}`);
			}
		},
		$$slots: { default: true }
	});
	$.bind_props($$props, { name });
}
