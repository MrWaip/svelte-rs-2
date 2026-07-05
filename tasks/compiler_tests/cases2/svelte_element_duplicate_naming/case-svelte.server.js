import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { tag = "div" } = $$props;
	let title = "hello";
	$.element($$renderer, tag, () => {
		$$renderer.push(` class="first"`);
	}, () => {
		$$renderer.push(`First: hello`);
	});
	$$renderer.push(` `);
	$.element($$renderer, tag, () => {
		$$renderer.push(` class="second"`);
	}, () => {
		$$renderer.push(`Second: hello`);
	});
}
