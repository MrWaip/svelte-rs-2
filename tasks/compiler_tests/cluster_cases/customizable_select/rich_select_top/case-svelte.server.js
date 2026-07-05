import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	let label = "hi";
	$$renderer.push(`<select><span class="hdr">${$.escape(label)}</span>`);
	$$renderer.option({ value: "a" }, ($$renderer) => {
		$$renderer.push(`A`);
	});
	$$renderer.push(`<!></select> <button>x</button>`);
}
