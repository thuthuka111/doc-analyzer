<script lang="ts">
	import type { ComparisonLogicalStructure } from '$types/ComparisonLogicalStructure';

	interface CollapseItem {
		collapsed: boolean;
		substructs?: CollapseItem[];
	}

	export let data: ComparisonLogicalStructure[] = [];
	export let ref_file_name: string;
	export let compare_file_name: string;
	let collapse_arr: CollapseItem[] = [];

	$: collapse_arr = createCollapseArr(data);

	/// replicated from LogViewTable.svelte
	function toggleSection(...args: number[]) {
		let i = args[0];
		let j = args.length > 1 ? args[1] : undefined;

		if (j === undefined) {
			collapse_arr[i].collapsed = !collapse_arr[i].collapsed;
		} else {
			if (collapse_arr[i].substructs) {
				collapse_arr[i].substructs[j].collapsed = !collapse_arr[i].substructs[j].collapsed;
			}
		}
	}

	// replicated from LogViewTable.svelte
	function determineVisibility(collapse_arr: CollapseItem[], ...args: number[]): boolean {
		let i = args[0];
		let j = args.length > 1 ? args[1] : undefined;

		if (j === undefined) {
			return collapse_arr[i].collapsed;
		} else {
			if (collapse_arr[i].substructs) {
				return collapse_arr[i].substructs[j].collapsed;
			} else {
				return false;
			}
		}
	}

	// replicated from LogViewTable.svelte
	function createCollapseArr(structure: ComparisonLogicalStructure[]): CollapseItem[] {
		return structure.map((item) => ({
			collapsed: false,
			substructs: item.substructure_differences
				? createCollapseArr(item.substructure_differences)
				: undefined
		}));
	}
</script>

<table class="table table-bordered">
	{#each data as structure, i}
		<thead>
			<tr class="table-section-header" on:click={() => toggleSection(i)}>
				<th>{structure.comp_structure?.name}</th>
				<th>{ref_file_name} value</th>
				<th>{compare_file_name} value</th>
			</tr>
		</thead>
		<tbody
			id="section2"
			class="table-section-content"
			class:show={determineVisibility(collapse_arr, i)}
		>
			{#if structure.ref_structure && structure.comp_structure}
				{#each structure.ref_structure.structure as structure_item, j}
					<tr style="max-width: 600px">
						<td>
							{structure_item.name}
							<div class="tooltip-container">
								<i style="cursor: pointer;">&#9432;</i>
								{#if structure_item.description}
									<span class="tooltip-text">{structure_item.description}</span>
								{:else}
									<span class="tooltip-text">No description available</span>
								{/if}
							</div>
						</td>
						<td style="max-width: inherit; word-wrap:break-word">{structure_item.value}</td>
						{#if structure.structure_differences[j]}
							<td class="diff-background" style="max-width: inherit; word-wrap:break-word">
								{structure.comp_structure.structure[j].value}
							</td>
						{:else}
							<td style="max-width: inherit; word-wrap:break-word"
								>{structure.comp_structure.structure[j].value}</td
							>
						{/if}
					</tr>
				{/each}
				<!-- this should not happen for the top level structs -->
			{/if}
			{#if structure.substructure_differences}
				{#each structure.substructure_differences as substructure, j}
					<tr class="p-0">
						<td class="p-0" colspan="3">
							<table
								class="table table-bordered"
								style="width: 98%; margin-left: 10px; margin-bottom: 0px;"
							>
								<thead>
									<tr class="table-section-header" on:click={() => toggleSection(i, j)}>
										<th class="even" colspan="1">
											{#if substructure.ref_structure}
												{substructure.ref_structure.name}
											{:else if substructure.comp_structure}
												{substructure.comp_structure.name}
											{/if}
											{#if substructure.structure_differences.length > 0}
												<span class="blue-dot"></span>
											{/if}
										</th>
										{#if substructure.ref_structure && !substructure.comp_structure}
											<th class="even" colspan="1"></th>
											<th class="even" colspan="1">{ref_file_name} value</th>
										{:else if !substructure.ref_structure && substructure.comp_structure}
											<th class="even" colspan="1">{compare_file_name} value</th>
											<th class="even" colspan="1"></th>
										{:else}
											<th class="even" colspan="1">{ref_file_name} value</th>
											<th class="even" colspan="1">{compare_file_name} value</th>
										{/if}
									</tr>
								</thead>
								<tbody
									id="section2"
									class="table-section-content"
									class:show={determineVisibility(collapse_arr, i, j)}
								>
									{#if substructure.ref_structure && substructure.comp_structure}
										{#each substructure.ref_structure.structure as structure_item, j}
											<tr>
												<td>
													{structure_item.name}
													<div class="tooltip-container">
														<i style="cursor: pointer;">&#9432;</i>
														{#if structure_item.description}
															<span class="tooltip-text">{structure_item.description}</span>
														{:else}
															<span class="tooltip-text">No description available</span>
														{/if}
													</div>
												</td>
												<td>{structure_item.value}</td>
												{#if substructure.structure_differences[j]}
													<td class="diff-background"
														>{substructure.comp_structure.structure[j].value}</td
													>
												{:else}
													<td>{substructure.comp_structure.structure[j].value}</td>
												{/if}
											</tr>
										{/each}
									{:else if substructure.ref_structure}
										{#each substructure.ref_structure.structure as structure_item, j}
											<tr>
												<td>{structure_item.name}</td>
												<td>
													{structure_item.name}
													<div class="tooltip-container">
														<i style="cursor: pointer;">&#9432;</i>
														{#if structure_item.description}
															<span class="tooltip-text">{structure_item.description}</span>
														{:else}
															<span class="tooltip-text">No description available</span>
														{/if}
													</div>
												</td>
												<td>{structure_item.value}</td>
												<td class="grey-diagonal-background"></td>
											</tr>
										{/each}
									{:else if substructure.comp_structure}
										{#each substructure.comp_structure.structure as structure_item, j}
											<tr>
												<td>
													{structure_item.name}
													<div class="tooltip-container">
														<i style="cursor: pointer;">&#9432;</i>
														{#if structure_item.description}
															<span class="tooltip-text">{structure_item.description}</span>
														{:else}
															<span class="tooltip-text">No description available</span>
														{/if}
													</div>
												</td>
												<td class="grey-diagonal-background"></td>
												<td>{structure_item.value}</td>
											</tr>
										{/each}
									{/if}
								</tbody>
							</table>
						</td>
					</tr>
				{/each}
			{/if}
		</tbody>
	{/each}
</table>

<!-- should really not be duplicating styles here -->
<style>
	.table-section-header {
		background-color: #808080;
		color: white;
		cursor: pointer;
	}
	.table-section-content {
		display: none;
		background-color: #d3d3d3;
	}
	.diff-background {
		background-color: rgba(49, 34, 255, 0.319);
	}
	.grey-diagonal-background {
		background-color: rgba(128, 128, 128, 0.5); /* Light grey with opacity */
		background-image: linear-gradient(
			135deg,
			rgba(255, 255, 255, 0.2) 25%,
			transparent 25%,
			transparent 50%,
			rgba(255, 255, 255, 0.2) 50%,
			rgba(255, 255, 255, 0.2) 75%,
			transparent 75%,
			transparent
		);
		background-size: 20px 20px; /* Size of the diagonal pattern */
	}
	.table-section-content > tr > td {
		padding: 0.2rem;
	}
	.table-section-content.show {
		display: table-row-group;
	}
	.even {
		width: 33%;
	}

	.tooltip-container {
		position: relative;
		display: inline-block;
	}

	.tooltip-text {
		visibility: hidden;
		width: 200px;
		background-color: #333;
		color: #fff;
		text-align: center;
		border-radius: 5px;
		padding: 5px;
		position: absolute;
		z-index: 1;
		bottom: 125%; /* Position the tooltip above the text */
		left: 150%;
		margin-left: -100px; /* Center the tooltip */
		opacity: 0;
		transition: opacity 0.3s;
	}

	.tooltip-container:hover .tooltip-text {
		visibility: visible;
		opacity: 1;
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
