import * as $ from "svelte/internal/server";
import { tooltip } from "./actions.js";
import { fade } from "svelte/transition";
export default function App($$renderer) {
	let value = "";
	$$renderer.push(`<input${$.attr("value", value)}/>`);
}
