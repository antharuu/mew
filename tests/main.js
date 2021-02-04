const Mew = require("../src/Mew")

Mew.Config = {
    entry_file: "./tests/index",
    variables: {
        bonjour: "Hello world from Mew! ♥",
        fruits: [
            "bananes",
            "pommes",
            "poires",
            "fraises"
        ]
    }
}

Mew.Compile()
