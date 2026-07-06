import * as $ from "svelte/internal/server";
import { tooltip } from "./actions.js";
export default function App($$renderer, $$props) {
	let { config, value } = $$props;
	$$renderer.push(`<label><input type="checkbox"/> <span>${$.escape(value)}</span></label>`);
}
