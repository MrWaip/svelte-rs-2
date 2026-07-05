import * as $ from "svelte/internal/server";
export default function App($$renderer, $$props) {
	let { prefix, suffix } = $$props;
	$$renderer.push(`<video src="x.mp4"${$.attr("muted", `${$.stringify(prefix)}${$.stringify(suffix)}`, true)} autoplay=""></video>`);
}
