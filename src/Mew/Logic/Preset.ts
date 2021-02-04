import {BlockElement} from "./BlockElement";
import {IAction} from "../Interfaces/IAction";

export class Preset {
    tag: string
    output: BlockElement
    callback: IAction

    constructor(
        tag: string,
        output: BlockElement = new BlockElement(),
        callback: IAction = (r: BlockElement, old: BlockElement) => r
    ) {
        this.tag = tag
        this.output = output
        this.callback = callback as IAction
    }
}