import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<div data-a="&amp;amp=q &lt; ">a</div> <div data-b="© &amp;reg=x > foo">b</div> <div data-c="&amp;ok &amp;=q">c</div>`);
}
