import * as $ from "svelte/internal/server";
import { foo } from "./foo";
export default function App($$renderer) {
	// this comment should move
	let count = 0;
	$$renderer.push(`<p>0</p>`);
}
