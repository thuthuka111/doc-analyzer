import { goto } from "$app/navigation";
import { invoke } from "@tauri-apps/api";

export async function analyseFile(path: string) {
	try {
		await invoke('read_file', { filePath: path });
		goto('/analyse');
	} catch (error) {
		alert('ERror from rust: ' + error);
	}
}


export async function analysieForCompare(path: string) {
	try {
		await invoke('read_file', { filePath: path });
		goto('/analyse');
	} catch (error) {
		alert('Error from rust: ' + error);
	}
}


export function ordinal_suffix_of(i: number | bigint) {
	const j = typeof i === 'bigint' ? i % BigInt(10) : i % 10,
		k = typeof i === 'bigint' ? i % BigInt(100) : i % 100;
	if (j == 1 && k != 11) {
		return "st";
	}
	if (j == 2 && k != 12) {
		return "nd";
	}
	if (j == 3 && k != 13) {
		return "rd";
	}
	return "th";
}