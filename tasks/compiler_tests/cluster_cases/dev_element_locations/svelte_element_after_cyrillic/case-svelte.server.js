import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { tag } = $$props;
	$$renderer.push(`<div>Текст `);
	$.element($$renderer, tag, void 0, () => {
		$$renderer.push(`жирный`);
	});
	$$renderer.push(`</div>`);
}
