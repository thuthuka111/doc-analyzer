<script lang="ts">
	import { ordinal_suffix_of } from '$lib/common';
	import type { ComparisonPhysicalStructure } from '$types/ComparisonPhysicalStructure';
	import { invoke } from '@tauri-apps/api';

	export let data: ComparisonPhysicalStructure[] = [];
	export let ref_file_name: string;
	export let compare_file_name: string;
	let interpret_utf8 = false;
	let collapse_arr: boolean[] = [true, false];
	let utf8_interpretations: [string, string][] = [];

	$: collapse_arr = Array(data.length).fill(false);
	$: if (data.length > 0) {
		set_utf8_interpretation();
	}

	function toggleSection(i: number) {
		collapse_arr[i] = !collapse_arr[i];
	}

	async function set_utf8_interpretation() {
		let ref_interpretations_res: string[] = await invoke('get_utf8_interpreted_bytes', {
			wordDoc: 'analysis'
		});
		let comp_interpretations_res: string[] = await invoke('get_utf8_interpreted_bytes', {
			wordDoc: 'comparison'
		});

		utf8_interpretations = ref_interpretations_res.map((ref, i) => [
			ref,
			comp_interpretations_res[i]
		]);
	}

	function comp_bolding(
		input: string,
		indice_arr: [number, number][],
		byte_level: boolean = false
	): string {
		// Handle empty array case
		if (indice_arr.length === 0) {
			return input;
		}
		// console.log(input, indice_arr);

		if (byte_level) {
			// return byte_level_bolding(input, indice_arr);
			// mpa the indices to the byte level by dividing and flooring each value
			indice_arr = indice_arr.map(([start, end]) => [Math.floor(start / 2), Math.floor(end / 2)]);
		}

		let html = '';
		let start = 0;

		for (let i = 0; i < indice_arr.length; i++) {
			let [start_index, end_index] = indice_arr[i];

			let pre_non_diff = input.slice(start, start_index);
			for (let j = 0; j < pre_non_diff.length; j++) {
				html += `<div class="text-center m-0" style="width: 11px">${pre_non_diff[j]}</div>`;
			}

			let diff = input.slice(start_index, end_index);
			for (let j = 0; j < diff.length; j++) {
				html += `<div class="text-center base-highlight m-0" style="width: 11px;">${diff[j]}</div>`;
			}

			start = end_index;
		}

		// Append the remaining part of the input string
		html += input.slice(start);

		return html;
	}

	function highlight_difference(
		input: string,
		indice_arr: [number, number][],
		byte_level: boolean = false
	): string {
		// indice_arr = [
		// 	[2, 4],
		// 	[10, 30],
		// 	[100, 113]
		// ];

		// Handle empty array case
		if (indice_arr.length === 0) {
			return input;
		}
		// console.log(input, indice_arr);

		if (byte_level) {
			// return byte_level_bolding(input, indice_arr);
			// mpa the indices to the byte level by dividing and flooring each value
			indice_arr = indice_arr.map(([start, end]) => [Math.floor(start / 2), Math.floor(end / 2)]);
		}

		let html = '';
		let start = 0;

		for (let i = 0; i < indice_arr.length; i++) {
			let [start_index, end_index] = indice_arr[i];

			let pre_non_diff = input.slice(start, start_index);
			for (let j = 0; j < pre_non_diff.length; j++) {
				// <div class="text-center m-0" style="width: 10px">{byte}</div>
				html += `<div class="text-center m-0" style="width: 11px">${pre_non_diff[j]}</div>`;
			}
			// html += input.slice(start, start_index);

			let diff = input.slice(start_index, end_index);
			for (let j = 0; j < diff.length; j++) {
				html += `<div class="text-center diff-highlight m-0" style="width: 11px">${diff[j]}</div>`;
			}
			// html += `<span class="diff-highlight">${input.slice(start_index, end_index)}</span>`;

			start = end_index;
		}

		// Append the remaining part of the input string
		html += input.slice(start);

		return html;
	}
</script>

<label class="ml-5">
	<input
		type="checkbox"
		bind:checked={interpret_utf8}
		id="interpret-utf8"
		class="form-check form-check-input"
	/>
	Interpret bytes as UTF-8
</label>

<table class="table table-bordered" style="width: 100%; table-layout: fixed;">
	{#each data as comparison_structure, i}
		{@const ref_structure = comparison_structure.ref_structure}
		{@const comp_structure = comparison_structure.comp_structure}
		<thead>
			<tr class="table-section-header" on:click={() => toggleSection(i)}>
				<th
					colspan="2"
					style="text-align: center; align-content: center; color: white; padding: 0.3rem;"
				>
					<div class="d-flex justify-content-center align-items-center">
						{#if ref_structure.structure_name && ref_structure.description}
							{ref_structure.structure_name} - <em>{ref_structure.description}</em>
						{:else}
							{#if ref_structure.structure_name}
								{ref_structure.structure_name}
							{/if}
							{#if ref_structure.description}
								<em>{ref_structure.description}</em>
							{/if}
						{/if}
						{#if comparison_structure.difference_indices.length > 0}
							<span class="blue-dot"></span>
						{/if}
					</div>
				</th>
			</tr>
		</thead>
		<tbody id="section2" class="table-section-content" class:show={collapse_arr[i]}>
			<tr>
				<td class="text-center p-0">
					{ref_structure.start_index}<sup>{ordinal_suffix_of(ref_structure.start_index)}</sup> - {ref_structure.end_index}<sup
						>{ordinal_suffix_of(ref_structure.end_index)}</sup
					>
					byte in <sub>in <em>{ref_structure.stream_name}</em></sub> | <em>{ref_file_name}</em>
				</td>
				<td class="text-center p-0">
					{comp_structure.start_index}<sup>{ordinal_suffix_of(comp_structure.start_index)}</sup> - {comp_structure.end_index}<sup
						>{ordinal_suffix_of(comp_structure.end_index)}</sup
					>
					byte in <sub>in <em>{comp_structure.stream_name}</em></sub> | <em>{compare_file_name}</em>
				</td>
			</tr>
			<!-- <tbody id="section2" class="table-section-content" class:show={i == 0 ? true : false}> -->
			<tr class="p-1">
				{#if interpret_utf8}
					<td colspan="1" class="hex-text">
						<div class="d-flex flex-wrap">
							{@html comp_bolding(
								utf8_interpretations[i][0],
								comparison_structure.difference_indices,
								true
							)}
						</div>
					</td>
					<td colspan="1" class="hex-text">
						<div class="d-flex flex-wrap">
							{@html highlight_difference(
								utf8_interpretations[i][1],
								comparison_structure.difference_indices,
								true
							)}
						</div>
					</td>
				{:else}
					<td colspan="1" class="hex-text">
						<div class="d-flex flex-wrap">
							{@html comp_bolding(ref_structure.bytes, comparison_structure.difference_indices)}
						</div>
					</td>
					<td colspan="1" class="hex-text">
						<div class="d-flex flex-wrap">
							{@html highlight_difference(
								comp_structure.bytes,
								comparison_structure.difference_indices
							)}
						</div>
					</td>
				{/if}
			</tr>
		</tbody>
	{/each}
</table>

<style>
	:global(.base-highlight) {
		/* color: rgb(49, 34, 255); Change this to the color you prefer */
		background-color: rgba(34, 255, 170, 0.205);
		font-weight: bold;
	}
	:global(.diff-highlight) {
		/* color: rgb(49, 34, 255); Change this to the color you prefer */
		background-color: rgba(49, 34, 255, 0.452);
		font-weight: bold;
	}
	sup {
		font-weight: 500;
	}
	.table-section-header {
		background-color: #808080;
		color: white;
		cursor: pointer;
	}
	.table-section-content {
		display: none;
		background-color: #d3d3d3;
	}
	.table-section-content.show {
		display: table-row-group;
	}

	/* Ensure table takes up full width */
	table {
		width: 100%;
		table-layout: fixed; /* Ensures proper wrapping */
	}

	/* Ensure hex text wraps inside the table cells */
	.hex-text {
		font-family: monospace;
		word-wrap: break-word;
		overflow-wrap: break-word;
		white-space: normal;
	}
	.blue-dot {
		display: inline-block;
		width: 10px;
		height: 10px;
		background-color: red;
		border-radius: 50%;
		margin-left: 5px;
	}
</style>
