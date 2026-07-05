import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		let { card } = $$props;
		function fmt(x) {
			return x;
		}
		$$renderer.push(`<span>${$.escape(card.flag ? "A" : "B" + fmt(card.x))} • ${$.escape(card.tail)}</span>`);
	});
}
