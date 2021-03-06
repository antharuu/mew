<p align="center">
  <img width="200" src="https://i.postimg.cc/XJZbJQRp/Logo.png" alt="Bubblegum icon">
</p>

#### Changelog: [Here](https://github.com/antharuu/Mew/blob/master/CHANGELOG.md)

Todo before 1.0:

- ✓ A similar base to PUG
- ✓ Add a preset system
- ✓ Custom presets
- ✓ Adding variables
    - ✓ String
    - ✓ Int
    - ✓ Float
    - ✓ Array
    - ✓ Objects
- Adding mixins
- Adding loops
    - While
    - ✓ For _(only for in for now)_
- Adding conditions
- Adding includes
    - Includes
    - Templates

...And probably a lot of other things

--- 

### Instalation

```
npm i mewjs
```

### Usage:

```js
const Mew = require("mew")

Mew.Render()
```

OR

```js
const Mew = require("mew")

Mew.RenderFile('./src/index')
```

OR

```js
const Mew = require("mew")

Mew.Config = {
    entry_file: "./src/index",
    variables: {
        hello: "Hello world from Mew! ♥"
    }
}

Mew.Compile()
```

### Exemple:

Now able to transform this

```pug
$myCss = "css/main.css"
$fruits = ["poires", "pommes", "pateques", "framboises"]
$user = {name: "Paul", age: 48, is_online: false}

doctype
html
  head
    viewport
    charset utf-8
    css {{myCss}}
    title {{bonjour}}
  body
    $bonjour = "Hello world"
    .container
      for fruit in fruits
        h2 J'adore les {{fruit}}
            a # Voir ce fruit
      .row.justify-contents-center
        .col-6
          h1 {{bonjour}}
          div#maSuperImage
            img#catImage https://unsplash.com/photos/_Kbydj4K7W8 Cat super image!
        section#main.col-6
          h2 Enjoy the new MEW preprocessor!
            fa b github
            | on github too!
          button:disabled My super button
          p check here
            a {{github}} Mew on Github
            |  if you want
```

To this:

```html
<!DOCTYPE html>
<html>

<head>
  <meta name="viewport" content="width=device-width, user-scalable=no, initial-scale=1.0, maximum-scale=1.0, minimum-scale=1.0">
  <meta charset="utf-8">
  <link rel="stylesheet" href="css/main.css">
  <title>Hello world from Mew! ♥</title>
</head>

<body>
<div class="container">
  <h2>J'adore les poires</h2>
  <h2>J'adore les pommes</h2>
  <h2>J'adore les pateques</h2>
  <h2>J'adore les framboises</h2>
  <div class="row justify-contents-center">
    <div class="col-6">
      <h1>Hello world</h1>
      <div id="maSuperImage"><img id="catImage" src_old="https://unsplash.com/photos/_Kbydj4K7W8" alt="Cat super image!"></div>
    </div>
    <section id="main" class="col-6">
      <h2>Enjoy the new MEW preprocessor!<fa>b github</fa>on github too!</h2><button disabled>My super button</button>
      <p>check here<a href="https://github.com/antharuu/Mew">Mew on Github</a> if you want</p>
    </section>
  </div>
</div>
</body>

</html>
```

> Beware the syntax may still have to change partially on some things.

### Custom presets

You can create custom presets to make your life easier later on.

You just have to pass presets this way in the options.

```js
presets: [
    {
        tag: "fa",
        element: {
            tag: "i",
            attributes: {
                class: "fa-icons"
            }
        },
        callback(newElement, oldElement) {
            const oldContent = oldElement.content.split(" ");
            let type = "s"
            if (oldContent.length >= 2) {
                type = oldContent[0]
                oldContent.shift()
            }
            oldContent.join(" ")
            newElement.attributes = {
                ...newElement.attributes, class: [
                    `fa${type}`,
                    `fa-${oldContent}`
                ]
            }
            return newElement; // Dont forget to return the new Element.
        }
    }
]
```

It will transform

```pug
fa b github

fa box-open
```

in

```html
<i class="fab fa-github"></i>

<i class="fas fa-box-open"></i>
```

> Customization with presets is up to you, you have no limits.
>
> We also use presets for many tasks.