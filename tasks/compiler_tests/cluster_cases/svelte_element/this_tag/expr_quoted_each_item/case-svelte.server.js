import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let tags = ["div", "span"];
	function bump() {
		tags = ["p"];
	}
	$$renderer.push(`<button>go</button> <!--[-->`);
	const each_array = $.ensure_array_like(tags);
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let t = each_array[$$index];
		$.element($$renderer, t, void 0, () => {
			$$renderer.push(`hello`);
		});
	}
	$$renderer.push(`<!--]-->`);
}
