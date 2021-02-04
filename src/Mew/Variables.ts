import {DataType} from "./Types/DataType";
import {BlockElement} from "./Logic/BlockElement";
import {Parser} from "./Parser";
import {Config} from "../Mew";

export module Variables {
    export let Data: DataType = {}
    export let DataBlock: DataType = {}

    function getData() {
        return {...Data, ...DataBlock}
    }

    let Checkers = [
        { // Replace
            callback: (str: string) => {
                for (const [key, value] of Object.entries(getData())) {
                    const regex = new RegExp("{{([ ]+)?(" + key + ")([ ]+)?}}", "g")
                    str = str.replace(regex, value);
                }
                return str;
            }
        },
        { // Set or Change value
            callback: (str: string) => {
                const regex = /\$([\w]+)[ ]*?([=])(.*)/
                str = str.trim()
                let m: RegExpExecArray;
                while ((m = regex.exec(str)) !== null) {
                    if (m.index === regex.lastIndex) regex.lastIndex++;
                    let vName: string = m[1]
                    let vValue: string = m[3].trim()
                    if (vValue.charAt(0) === '"') vValue = vValue.substr(1, vValue.length - 2)
                    Data[vName] = vValue;
                    str = ""
                }
                return str;
            }
        },
        { // For in
            callback: (str: string) => {
                const regex = /for ([\w]+) in ([\w]+)/
                str = str.trim()

                let m: RegExpExecArray;
                while ((m = regex.exec(str)) !== null) {
                    if (m.index === regex.lastIndex) regex.lastIndex++;
                    let vIteration: string | any = m[2]
                    let vValue: string = m[1]
                    for (const [key, value] of Object.entries(getData())) {
                        if (key === vIteration) {
                            str = `loop-for(iteration="${vIteration}" value="${vValue}")`
                        }
                    }
                }
                return str;
            }
        }
    ]

    export const check = (str: string) => {
        Checkers.forEach(checker => str = checker.callback(str))
        return str;
    };

    export const checkBlock = (block: BlockElement): BlockElement => {
        const line = check(block.line)
        const P = new Parser(line, Config)
        return P.Blocks[0];
    };
}