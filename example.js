const fsParser  = require('.')
const path = require('node:path')
const fs   = require('node:fs')

const start = new Date();
const dirName = "C:\\Users\\jtsag\\Documents\\My Games\\FarmingSimulator2022\\mods\\fsg_realism"
const fileList = fs.readdirSync(dirName)

for ( const file of fileList ) {
	const fullPath = path.join(dirName, file)
	const modObject = JSON.parse(fsParser.parseModFirstPass(fullPath));
	console.log(modObject)
}

const end = new Date();
console.log((end.getTime() - start.getTime())/1000, "s");