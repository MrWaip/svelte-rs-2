import * as $ from "svelte/internal/server";
export default function App($$renderer) {
	$$renderer.push(`<!---->some text ${$.escape(some_varaible)} after text`);
}
