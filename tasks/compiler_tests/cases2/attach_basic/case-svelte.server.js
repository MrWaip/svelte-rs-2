import * as $ from "svelte/internal/server";
import { tooltip } from "./actions.js";
export default function App($$renderer) {
	$$renderer.push(`<div>hello</div>`);
}
