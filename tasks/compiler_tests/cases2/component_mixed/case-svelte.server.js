import * as $ from "svelte/internal/server";
import Button from "./Button.svelte";
import Icon from "./Icon.svelte";
export default function App($$renderer) {
	$$renderer.push(`<h1>Title</h1> `);
	Button($$renderer, {});
	$$renderer.push(`<!----> `);
	Icon($$renderer, {});
	$$renderer.push(`<!---->`);
}
