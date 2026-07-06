import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<section><span><span>${$.escape(name)}</span></span> <div><div><div>text</div></div></div> <p><b><i${$.attr("name", name)}></i></b></p></section>`);
}
