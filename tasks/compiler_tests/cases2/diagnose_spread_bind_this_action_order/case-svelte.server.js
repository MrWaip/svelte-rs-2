import * as $ from "svelte/internal/server";
import { act } from "./act";
export default function App($$renderer) {
	let ref;
	let attrs = {};
	$$renderer.push(`<div${$.attributes({ ...attrs })}></div>`);
}
