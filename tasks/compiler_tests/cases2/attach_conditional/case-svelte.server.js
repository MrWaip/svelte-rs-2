import * as $ from "svelte/internal/server";
import { tooltip } from "./actions.js";
export default function App($$renderer) {
	let enabled = true;
	$$renderer.push(`<div>hello</div>`);
}
