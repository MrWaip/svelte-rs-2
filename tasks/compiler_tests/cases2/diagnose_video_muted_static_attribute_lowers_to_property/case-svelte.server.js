import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<video src="x.mp4" muted="" autoplay=""></video>`);
}
