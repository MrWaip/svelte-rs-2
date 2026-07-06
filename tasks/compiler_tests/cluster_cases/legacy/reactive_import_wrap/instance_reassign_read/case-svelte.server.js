import * as $ from "svelte/internal/server";
import { numbers } from "./data.js";
export default function App($$renderer, $$props) {
	$$renderer.component(($$renderer) => {
		function add() {
			numbers[numbers.length] = numbers.length + 1;
		}
		$$renderer.push(`<p>${$.escape(numbers.join(" + "))}</p> <button>add</button>`);
	});
}
