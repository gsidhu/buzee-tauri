import { sendEvent } from '../../utils/firebase';

export async function startFileScan() {
	const result = await window.electronAPI.startFileScan();
	return result;
}

export function selectAllRows(invert: boolean = false) {
	if (typeof document === 'undefined') return;
	const rows = document.getElementsByClassName('table-row');
	for (let i = 0; i < rows.length; i++) {
		if (invert) (rows[i] as HTMLDivElement).classList.remove('selected');
		else (rows[i] as HTMLDivElement).classList.add('selected');
	}
	return;
}

export function selectOneRow(row: HTMLDivElement) {
	row.classList.add('selected');
	row.focus();
	return;
}

export function clickRow(
	e:
		| (MouseEvent & { currentTarget: EventTarget & HTMLDivElement })
		| (FocusEvent & { currentTarget: EventTarget & HTMLDivElement }),
	shiftKeyPressed: boolean
) {
	const row = e.currentTarget;
	if (!row) return;
	// check if row has table-row class, if not, return
	if (!row.classList.contains('table-row')) {
		return;
	} else if (row.classList.contains('selected')) {
		return;
	}
	// check if shift key is pressed, if so, select all rows between this row and the last selected row
	const rows = document.getElementsByClassName('table-row');
	if (shiftKeyPressed) {
		const selectedRow = document.querySelector('.table-row.selected');
		if (selectedRow) {
			const selectedIndex = Array.from(rows).indexOf(selectedRow);
			const clickedIndex = Array.from(rows).indexOf(row);
			const start = Math.min(selectedIndex, clickedIndex);
			const end = Math.max(selectedIndex, clickedIndex);
			for (let i = start; i <= end; i++) {
				(rows[i] as HTMLDivElement).classList.add('selected');
			}
		} else {
			row.classList.add('selected');
		}
	} else {
		for (let i = 0; i < rows.length; i++) {
			rows[i].classList.remove('selected');
			(rows[i] as HTMLDivElement).blur();
		}
		row.classList.add('selected');
		(row as HTMLDivElement).focus();
	}
	sendEvent('click:clickRow');
	return;
}
