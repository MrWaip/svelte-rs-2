import * as $ from "svelte/internal/server";
import customElementWrapper from "./wrapper.js";
export default function App($$renderer, $$props) {
	let { name } = $$props;
	$$renderer.push(`<p>${$.escape(name)}</p>`);
}
