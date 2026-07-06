import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { tag = "p", name } = $$props;
	$.element($$renderer, tag, void 0, () => {
		$$renderer.push(`Hello ${$.escape(name)}!`);
	});
}
