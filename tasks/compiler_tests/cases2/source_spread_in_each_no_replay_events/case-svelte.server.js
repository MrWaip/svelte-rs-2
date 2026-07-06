import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { sources } = $$props;
	$$renderer.push(`<picture><!--[-->`);
	const each_array = $.ensure_array_like(sources);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let source = each_array[$$index];
		$$renderer.push(`<source${$.attributes({ ...source })}/>`);
	}
	$$renderer.push(`<!--]--></picture>`);
}
