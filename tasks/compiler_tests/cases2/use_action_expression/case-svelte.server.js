import * as $ from "svelte/internal/server";
import { tooltip } from "./actions.js";
export default function App($$renderer) {
	let config = { text: "hello" };
	$$renderer.push(`<div>text</div>`);
}
