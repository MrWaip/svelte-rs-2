import "svelte/internal/flags/async";
import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like({ length: 1 });
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		const data = 1;
		$$renderer.push(`<!---->1`);
	}
	$$renderer.push(`<!--]--> `);
	const each_array_1 = $.ensure_array_like({ length: 0 });
	if (each_array_1.length !== 0) {
		$$renderer.push("<!--[-->");
		for (let $$index_1 = 0, $$length = each_array_1.length; $$index_1 < $$length; $$index_1++) {
			$$renderer.push(`<!---->x`);
		}
	} else {
		$$renderer.push("<!--[!-->");
		const data = 2;
		$$renderer.push(`<!---->2`);
	}
	$$renderer.push(`<!--]-->`);
}
