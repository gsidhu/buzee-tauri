export function stringToHash(input: string) {
  var hash = 0;
  if (input.length === 0) {
    return hash.toString().padStart(10, '0'); // return 10 digit hash with leading zeros if input is empty
  }
  for (var i = 0; i < input.length; i++) {
    var char = input.charCodeAt(i);
    hash = (hash << 5) - hash + char;
    hash = hash & hash; // convert to 32bit integer
  }
  return Math.abs(hash).toString().slice(0, 10).padStart(10, '0'); // return 10 digit hash with leading zeros
}

export function readableFileSize(fileSize: number): string {
	if (!fileSize) return '';
	// convert fileSize to KB, MB, GB
	if (fileSize < 1000) {
		return fileSize + ' B';
	} else if (fileSize < 1000000) {
		return (fileSize / 1000).toFixed(2) + ' KB';
	} else if (fileSize < 1000000000) {
		return (fileSize / 1000000).toFixed(2) + ' MB';
	}
	return (fileSize / 1000000000).toFixed(2) + ' GB';
}

export function resetColumnSize() {
  const resizers = document.querySelectorAll('.resizer');
  resizers.forEach((el) => {
    var clickEvent = document.createEvent ('MouseEvents');
    clickEvent.initEvent ('dblclick', true, true);
    el.dispatchEvent (clickEvent);
  });
}

export function categoriseExtensions(received_filetypes: FileTypes[]) {
  let allowedExtensions: FileTypesDropdown = {
		categories: [],
		items: []
	};
  received_filetypes.forEach((extension) => {
    if (allowedExtensions.categories.indexOf(extension.file_type_category) === -1) {
      allowedExtensions.categories.push(extension.file_type_category);
    }
  });
  allowedExtensions.items = received_filetypes;
  // sort allowedExtensions.categories alphabetically
  allowedExtensions.categories.sort();
  return allowedExtensions;
}

export function setExtensionCategory(extension: string | undefined, allowedExtensions: FileTypesDropdown) {
  console.log("extension:", extension);
  
  if (extension === 'any') {
    return extension;
  }
  let fileTypeString = "";
  // For each item in allowedExtensions.items, check if item.file_type_category = extension
  // If so, add it to fileTypeString
  allowedExtensions.items.forEach((item) => {
    if (item.file_type_category === extension) {
      fileTypeString += item.file_type + ",";
    }
  });
  // Remove trailing comma
  fileTypeString = fileTypeString.slice(0, -1);
  return fileTypeString;
}