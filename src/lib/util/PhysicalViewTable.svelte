<script lang="ts">
	import { ordinal_suffix_of } from '$lib/common';
	import type { PhysicalStructure } from '$types/PhysicalStructure';
	import { invoke } from '@tauri-apps/api';

	export let data: PhysicalStructure[] = [];
	let collapse_arr: boolean[] = [true, false];
	let interpret_utf8 = false;
	let utf8_interpretations: string[] = [];

	$: collapse_arr = Array(data.length).fill(false);
	$: if (data.length > 0) {
        set_utf8_interpretation();
    }


	function toggleSection(i: number) {
		collapse_arr[i] = !collapse_arr[i];
	}

	async function set_utf8_interpretation() {
		let interpretations_res: string[] = await invoke('get_utf8_interpreted_bytes', {
			wordDoc: 'analysis'
		});
		console.log(interpretations_res);
		utf8_interpretations = interpretations_res;
	}

</script>

<!-- <div class=""> -->
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
		{#each data as structure, i}
			<thead>
				<tr class="table-section-header" on:click={() => toggleSection(i)}>
					<th
						colspan="3"
						style="text-align: center; align-content: center; color: white; padding: 0.3rem;"
					>
						{structure.start_index}<sup>{ordinal_suffix_of(structure.start_index)}</sup> - {structure.end_index}<sup
							>{ordinal_suffix_of(structure.end_index)}</sup
						>
						byte in <sub>in <em>{structure.stream_name}</em></sub>
						{#if structure.structure_name}({structure.structure_name})
						{/if}
					</th>
				</tr>
			</thead>
			<tbody id="section2" class="table-section-content" class:show={collapse_arr[i]}>
				<tr class="p-1">
					{#if structure.description}
						{#if !interpret_utf8}
							<td colspan="2" class="hex-text" style="letter-spacing: 0.1rem;">{structure.bytes}</td
							>
						{:else}
							<td colspan="2" class="hex-text" style="letter-spacing: 0.1rem;">
								{utf8_interpretations[i]}
							</td>
						{/if}
						<th colspan="1" style="font-size: small; font-weight: 400; padding: 0.2rem">
							{structure.description}
						</th>
					{:else}
						<td colspan="3" class="hex-text" style="letter-spacing: 0.1rem;">{structure.bytes}</td>
					{/if}
				</tr>
			</tbody>
		{/each}
	</table>
<!-- </div> -->

<style>
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
</style>
