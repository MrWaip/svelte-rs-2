import * as $ from "svelte/internal/server";
import { helper } from "./helper.js";
export default function App($$renderer, $$props) {
	$$renderer.push(`<p>ok</p>`);
	$.bind_props($$props, { helper });
}
