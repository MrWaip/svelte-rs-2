import "svelte/internal/flags/legacy";
import * as $ from "svelte/internal/client";
var root = $.from_html(`<template shadowrootmode="closed"><p><slot></slot></p></template>`);
export default function App($$anchor) {
	var template = root();
	$.append($$anchor, template);
}
