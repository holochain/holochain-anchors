// Get an anchors hash and make sure it's commited into the anchors system
function anchor(anchor){
	// Create the anchor type which is the same as this anchor but with blank text
	var anchorType = {anchorType: anchor.anchorType, anchorText: ''};
	// The root anchor type
	var rootAnchortype =  {anchorType: 'anchorTypes', anchorText: ''};
	// Get the address of the anchor
	var anchorHash = makeHash('anchor', anchor);
	// See if the anchor exists?
	// If it does just return the address
	var anchorGet = get(anchorHash);
	// Anchor doesn't exist
	if(anchorGet === null){
		/* Duplicates
		var anchorType = {anchorType: anchor.anchorType, anchorText: ''};
		var rootAnchortype =  {anchorType: 'anchorTypes', anchorText: ''};
		*/
		// Get the address of the anchor type and see if it exists
		var anchorTypeGet = get(makeHash('anchor', anchorType));
		if(anchorTypeGet === null){
			// Get the root anchors address
			var rootAnchorTypeHash = makeHash('anchor', rootAnchortype);
			// If the root anchor doesn't exist commit it
			if (get(rootAnchorTypeHash) === null){
				rootAnchorTypeHash = commit('anchor', rootAnchortype);
			}
			// Commit the anchor type 
			var anchorTypeHash = commit('anchor', anchorType);

			// Link from the root anchor to the anchor type with `anchor_link` as the link type and the anchor type's type as the tag.
			commit('anchor_link', { Links:[{Base: rootAnchorTypeHash, Link: anchorTypeHash, Tag: anchorType.anchorType}]});

		} else {
			// Anchor type exists store the hash (scoping is weird here?)
			anchorTypeHash = makeHash('anchor', anchorType);
		}
		// Commit the anchor
		anchorHash = commit('anchor', anchor);
		// Link from the anchor type to the anchor using the link type as `anchor_link` and the anchor text as a tag
		commit('anchor_link',  { Links:[{Base: anchorTypeHash, Link: anchorHash, Tag: anchor.anchorText}]});
	}
	return anchorHash;
}

function exists(anchor){
	var key = get(makeHash('anchor', anchor));
	if(key !== null){
		return true;
	}
	return false;
}

function anchors(type){
	var links = getLinks(makeHash('anchor', {anchorType: type, anchorText: ''}), '');
	return links;
}

function genesis() {
	return true;
}

function validatePut(entry_type,entry,header,pkg,sources) {
	return validateCommit(entry_type,entry,header,pkg,sources);
}
function validateCommit(entry_type,entry,header,pkg,sources) {
	if (entry_type == 'anchor') {
		return true;
	}
	if (entry_type == 'anchor_link') {
		return true;
	}
	return false;
}



function validateLink(linkingEntryType,baseHash,linkHash,pkg,sources){
	return true;
}
function validateMod(entry_type,hash,newHash,pkg,sources){
	return true;
}
function validateDel(entry_type,hash,pkg,sources) {
	return true;
}
function validatePutPkg(entry_type) {
	return null;
}
function validateModPkg(entry_type) {
	return null;
}
function validateDelPkg(entry_type) {
	return null;
}
function validateLinkPkg(entry_type) {
	return null;
}
