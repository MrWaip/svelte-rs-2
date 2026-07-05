import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<input type="text"/> <br/> <img src="test.png"/> <hr/>`);
}
