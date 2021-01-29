import {BlockElement} from "./Logic/BlockElement";
import {Presets} from "./Presets";

// noinspection JSUnusedLocalSymbols
const {['log']: cl} = console; // Personal shortcut TODO: remove later

const autoClosableTags = [
    "!DOCTYPE", "br", "hr", "meta", "area",
    "base", "col", "embed", "img", "input", "link",
    "param", "source", "track", "wbr", "command",
    "keygen", "menuitem"
]


function checkPresets(block: BlockElement) {
    let rBlock: BlockElement = block;
    Presets.forEach(preset => {
        if (block.tag === preset.tag) {
            const out = preset.output;
            rBlock = new BlockElement(out)
            if (preset.callback ?? false) rBlock = preset.callback(rBlock, block)
        }
    })
    return rBlock;
}

export const Htmlify = (blocks: Array<BlockElement>, i: number = 0) => {
    let finalCode = "";
    const indent = "    ".repeat(i);
    blocks.forEach(block => {

        if (block.tag !== "|") {
            block = checkPresets(block)
            finalCode += indent + "<" + block.tag;
            for (const [attribute, value] of Object.entries(block.attributes)) {
                if (value !== null) {
                    finalCode += " " + attribute + "=\""
                    let v = 0;
                    value.forEach(val => {
                        if (v !== 0) finalCode += " ";
                        finalCode += val;
                        v++;
                    })
                    finalCode += "\""
                } else {
                    finalCode += " " + attribute
                }
            }
            if (!autoClosableTags.includes(block.tag)) finalCode += ">";
            else finalCode += " />";
            finalCode += block.content.trim()
            finalCode += Htmlify(block.block, i + 1).trim() // <- Recursive
            if (!autoClosableTags.includes(block.tag)) finalCode += "</" + block.tag + ">";
        }else{
            finalCode += block.content
        }
    })

    return finalCode;
};

