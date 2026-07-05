import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let fruit = "apple";
	$$renderer.push(`<select><optgroup label="Fruits"><span class="hdr">${$.escape(fruit)}</span> `);
	$$renderer.option({ value: "a" }, ($$renderer) => {
		$$renderer.push(`<span>${$.escape(fruit)}</span> ${$.escape(fruit)}`);
	}, void 0, void 0, void 0, void 0, true);
	$$renderer.push(` `);
	$$renderer.option({ value: "b" }, ($$renderer) => {
		$$renderer.push(`banana`);
	});
	$$renderer.push(`<!></optgroup></select> <button>x</button>`);
}
