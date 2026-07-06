import * as $ from "svelte/internal/server";
import { handler } from "./module.js";
export default function App($$renderer) {
	$$renderer.push(`<button>Click</button>`);
}
