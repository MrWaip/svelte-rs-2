import * as $ from "svelte/internal/server";
import { custom } from "./transitions.js";
export default function App($$renderer) {
	$$renderer.push(`<div>hello</div>`);
}
