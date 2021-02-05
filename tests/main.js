const Mew = require("../src/Mew")

Mew.Config = {
    entry_file: "./tests/index",
    variables: {
        bonjour: "Hello world from Mew! ♥",
        amount: "5",
        fruits: [
            "bananes",
            "pommes",
            "poires",
            "fraises"
        ],
        user: {
            name: 'jean',
            age: 28,
            is_online: true
        }
    }
}

Mew.Compile()
