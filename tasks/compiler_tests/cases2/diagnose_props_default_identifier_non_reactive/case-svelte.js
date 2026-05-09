import * as $ from "svelte/internal/client";
import { noop } from "./helpers";
export default function App($$anchor, $$props) {
	let onError = $.prop($$props, "onError", 3, noop);
}
