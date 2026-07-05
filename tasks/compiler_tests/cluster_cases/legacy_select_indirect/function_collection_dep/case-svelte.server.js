import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let letters = $.fallback($$props["letters"], () => [
			"a",
			"b",
			"c"
		], true);
		let selected = $.fallback($$props["selected"], () => ({ letter: "" }), true);
		function uppercase() {
			return letters.map((x) => x.toUpperCase());
		}
		$$renderer.select({ value: selected.letter }, ($$renderer) => {
			$$renderer.push(`<!--[-->`);
			const each_array = $.ensure_array_like(uppercase());
			for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
				let letter = each_array[$$index];
				$$renderer.option({ value: letter }, ($$renderer) => {
					$$renderer.push(`${$.escape(letter)}`);
				});
			}
			$$renderer.push(`<!--]-->`);
		});
		$$renderer.push(` ${$.escape(selected.letter)}`);
		$.bind_props($$props, {
			letters,
			selected
		});
	});
}
