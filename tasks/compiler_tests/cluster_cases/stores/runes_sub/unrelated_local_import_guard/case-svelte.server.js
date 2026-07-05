import * as $ from "svelte/internal/server";
import { helper } from "./store.js";
export default function App($$renderer) {
	let a = 0;
	$$renderer.push(`<button>${$.escape(a)} ${$.escape(helper)}</button>`);
}
