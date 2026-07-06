import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let label = "hi";
	$$renderer.push(`<select><optgroup label="g"><em class="hdr">${$.escape(label)}</em> `);
	$$renderer.option({ value: "a" }, ($$renderer) => {
		$$renderer.push(`A`);
	});
	$$renderer.push(`<!></optgroup></select> <button>x</button>`);
}
