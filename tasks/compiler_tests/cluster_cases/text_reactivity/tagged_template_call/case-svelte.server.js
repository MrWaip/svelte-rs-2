import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	function tag(s) {
		return s[0];
	}
	$$renderer.push(`<p>v ${$.escape(tag``)}</p>`);
}
