import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let fruit = "apple";
	let veggie = "carrot";
	$$renderer.push(`<select><optgroup label="Fruits"><span class="fh">${$.escape(fruit)}</span> `);
	$$renderer.option({ value: "a" }, ($$renderer) => {
		$$renderer.push(`<span>${$.escape(fruit)}</span>`);
	}, void 0, void 0, void 0, void 0, true);
	$$renderer.push(`<!></optgroup><optgroup label="Vegs"><em class="vh">${$.escape(veggie)}</em> `);
	$$renderer.option({ value: "c" }, ($$renderer) => {
		$$renderer.push(`<em>${$.escape(veggie)}</em>`);
	}, void 0, void 0, void 0, void 0, true);
	$$renderer.push(`<!></optgroup></select> <button>x</button>`);
}
