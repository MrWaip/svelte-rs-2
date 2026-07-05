import * as $ from "svelte/internal/server";
import { snip } from "./snip.js";
export default function App($$renderer) {
	function getArg() {
		return 1;
	}
	snip($$renderer, getArg());
}
