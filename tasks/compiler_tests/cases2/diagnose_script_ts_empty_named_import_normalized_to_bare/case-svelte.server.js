import * as $ from "svelte/internal/server";
import "./side-effect.js";
export default function App($$renderer) {
	$$renderer.push(`<div></div>`);
}
