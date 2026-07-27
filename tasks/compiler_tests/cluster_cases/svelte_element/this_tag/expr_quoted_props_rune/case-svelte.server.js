import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { tag } = $$props;
	$.element($$renderer, tag, void 0, () => {
		$$renderer.push(`hello`);
	});
}
