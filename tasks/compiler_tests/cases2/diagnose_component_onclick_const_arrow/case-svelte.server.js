import * as $ from "svelte/internal/server";
import Btn from "./Btn.svelte";
export default function App($$renderer) {
	const save = () => {};
	Btn($$renderer, { onclick: save });
}
