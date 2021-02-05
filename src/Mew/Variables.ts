import {DataType} from "./Types/DataType";
import {BlockElement} from "./Logic/BlockElement";
import {Parser} from "./Parser";
import {Config} from "../Mew";
import {cl} from "../cl";
import {Variable} from "./Logic/Variable";

export module Variables {
    export let Data: DataType = []
    export let DataBlock: DataType = []

    function getData() {
        let save: { [key: string]: Variable } = {}
        let data: Variable[] = [...Data, ...DataBlock]
        data.forEach((v: Variable) => save[v.name] = v)
        data = []
        for (const [, value] of Object.entries(save)) data.push(value)
        return data
    }

    let Checkers = [
        { // --------------------------------------------- Replace
            callback: (str: string) => {
                getData().forEach((v: Variable) => {
                    const regex = new RegExp("{{([ ]+)?(" + v.name + ")([ ]+)?}}", "g")
                    str = str.replace(regex, v.value);
                })
                return str;
            }
        },
        { // --------------------------------------------- Set or Change value
            callback: (str: string) => {
                const regex = /\$([\w]+)[ ]*?([=])(.*)/
                str = str.trim()
                let m: RegExpExecArray;
                while ((m = regex.exec(str)) !== null) {
                    if (m.index === regex.lastIndex) regex.lastIndex++;
                    let vName: string = m[1]
                    let vValue: string = m[3].trim()
                    if (vValue.charAt(0) === '"') vValue = vValue.substr(1, vValue.length - 2)
                    Variables.addVariable(vName, vValue)
                    str = ""
                }
                return str;
            }
        },
        { // --------------------------------------------- For in
            callback: (str: string) => {
                const regex = /for ([\w]+) in ([\w]+)/
                str = str.trim()

                let m: RegExpExecArray;
                while ((m = regex.exec(str)) !== null) {
                    if (m.index === regex.lastIndex) regex.lastIndex++;
                    let vIteration: string | any = m[2]
                    let vValue: string = m[1]
                    getData().forEach((v: Variable) => {
                        if (v.name === vIteration) {
                            str = `loop-for(iteration="${vIteration}" value="${vValue}")`
                        }
                    })
                }
                return str;
            }
        }
    ]

    export const addVariable = (name: string, value: any, type: string | undefined = undefined) => {
        Data.push(new Variable(name, value, type))
    }

    export const addVariables = (variables: Object) => {
        for (let [key, value] of Object.entries(variables)) addVariable(key, value)
    }

    export const addBlockVariable = (name: string, value: any, type: string | undefined = undefined) => {
        DataBlock.push(new Variable(name, value, type))
    }

    export const addBlockVariables = (variables: Object) => {
        for (let [key, value] of Object.entries(variables)) addBlockVariable(key, value)
    }

    export const blockReset = () => {
        DataBlock = []
    }

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