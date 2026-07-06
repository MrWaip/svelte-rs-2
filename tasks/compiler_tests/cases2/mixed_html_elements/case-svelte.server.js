import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<input/> <textarea></textarea> <area/> <br/> <a></a>`);
}
