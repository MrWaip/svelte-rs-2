import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let cards = [{ fav: false }];
	$$renderer.push(`<!--[-->`);
	const each_array = $.ensure_array_like(cards.filter((c) => !c.fav));
	for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
		let card = each_array[$$index];
		$$renderer.push(`<button>${$.escape(card.fav)}</button>`);
	}
	$$renderer.push(`<!--]-->`);
}
