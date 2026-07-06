import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	const facts = [
		"Cats have five toes on their front paws, but only four on the back.",
		"A group of flamingos is called a 'flamboyance'.",
		"Bananas are berries, but strawberries aren't."
	];
	$$renderer.push(`<ol><!--[-->`);
	const each_array = $.ensure_array_like(facts);
	for (let i = 0, $$length = each_array.length; i < $$length; i++) {
		let fact = each_array[i];
		$$renderer.push(`<li>${$.escape(fact)}</li>`);
	}
	$$renderer.push(`<!--]--></ol>`);
}
