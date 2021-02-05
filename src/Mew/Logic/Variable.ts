const strToObject = require('object-string-to-object');

export class Variable {
    name: string
    value: any
    type: string | undefined = undefined

    constructor(name: string, value: any, type: string | undefined = undefined) {
        this.name = name
        this.value = value
        this.type = this.getTypeOf(type, value)
    }

    private getTypeOf(type: string, value: any): string | undefined {
        if (type === undefined) {
            if (typeof value === "string") type = this.regexTests(value);
            else if (typeof value === "object") {
                if (value[0] !== undefined) type = "array"
                else type = "object"
            } else type = typeof value;
        }
        if (typeof type === "string") type = type.toLowerCase()
        return type as string;
    }

    private regexTests(value: string) {
        let type = undefined;
        if (testNumbers(value)) type = testInt(value) ? "int" : "float"
        if (testArray(value)) {
            type = "array"
            this.value = JSON.parse(value)
        }
        if (testObject(value)) {
            type = "object"
            this.value = strToObject(value)
        }
        return type !== undefined ? type : "string";
    }
}


const testNumbers = (value: string): boolean => /^\d+(\.\d+)?$/g.test(value)

const testInt = (value: string): boolean => /^\d+$/g.test(value)

const testArray = (value: string): boolean => /^\[(.+)+?]$/g.test(value)

const testObject = (value: string): boolean => /^{(.+)+?}$/g.test(value)