import {BlockElement} from "../Logic/BlockElement";
import {Preset} from "../Logic/Preset";

const getAttributeBlockContent = (attrName: string, rBlock: BlockElement, oldBlock: BlockElement) => {
    rBlock.attributes[attrName] = [oldBlock.content]
    return rBlock
}

export const getPresetsFrom = (userPresets: Preset[]) => {
    let presets: Preset[] = []
    for (const preset of userPresets) presets.push(new Preset(preset.tag, preset.output, preset.callback))
    return presets;
}

export const Presets = [
    new Preset(
        "doctype",
        new BlockElement({tag: "!DOCTYPE", attributes: {html: null}})
    ),
    new Preset(
        "charset",
        new BlockElement({tag: "meta"}),
        (rBlock: BlockElement, oldBlock: BlockElement) => getAttributeBlockContent("charset", rBlock, oldBlock)
    ),
    new Preset(
        "css",
        new BlockElement({tag: "link", attributes: {rel: ["stylesheet"]}}),
        (rBlock: BlockElement, oldBlock: BlockElement) => getAttributeBlockContent("href", rBlock, oldBlock)
    ),
    new Preset(
        "a",
        new BlockElement({
            tag: "a",
        }),
        (rBlock: BlockElement, oldBlock: BlockElement) => {
            const c: string[] = oldBlock.content.split(" ");
            if (c.length < 2) throw "A link needs at least 2 arguments"
            rBlock.attributes = oldBlock.attributes
            rBlock.attributes["href"] = [c[0]]
            c.shift()
            rBlock.content = c.join(" ")
            return rBlock
        }
    ),
    new Preset(
        "img",
        new BlockElement({
            tag: "img",
        }),
        (rBlock: BlockElement, oldBlock: BlockElement) => {
            const c: string[] = oldBlock.content.split(" ");
            rBlock.attributes = oldBlock.attributes
            rBlock.attributes["src_old"] = [c[0]]
            if (c.length >= 2) {
                c.shift()
                rBlock.attributes["alt"] = [c.join(" ")]
            }
            return rBlock
        }
    ),
    new Preset(
        "viewport",
        new BlockElement({
            tag: "meta",
            attributes: {
                name: "viewport",
                content: [
                    "width=device-width,",
                    "user-scalable=no,",
                    "initial-scale=$size$,",
                    "maximum-scale=$size$,",
                    "minimum-scale=$size$",
                ]
            },
        }),
        (rBlock: BlockElement, oldBlock: BlockElement) => {
            if (oldBlock.content === "") oldBlock.content = "1.0";
            rBlock.attrReplace("content", "$size$", oldBlock.content)
            return rBlock
        }
    )
]