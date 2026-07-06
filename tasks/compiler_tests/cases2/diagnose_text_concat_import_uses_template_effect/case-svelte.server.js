import * as $ from "svelte/internal/server";
import { NAME } from "./lib";
import Other from "./Other.svelte";
export default function App($$renderer) {
	$$renderer.push(`<div>Hello ${$.escape(NAME)} world`);
	Other($$renderer, {});
	$$renderer.push(`<!----></div>`);
}
